mod config;
mod datetime;
mod fixtures;
mod scrape;

use chrono::{Datelike, Days, Months, NaiveDate, Utc};
use clap::Parser;
use config::AppConfig;
use fixtures::FixtureList;
use scrape::get_flist;

use appindicator3::prelude::AppIndicatorExt;
use appindicator3::Indicator;
use appindicator3::IndicatorStatus;
use gtk::prelude::*;
use gtk::{Menu, MenuItem};
use serde::ser::Error;
use std::thread;
use std::time::Duration;

const URL_BASE_BBC: &str =
    "https://www.bbc.com/sport/rugby-union/teams/PLACEHOLDER/scores-fixtures/";

#[derive(Parser, Debug)]
#[clap(author = "Charles Giese", version, about = "Get Rugby Fixtures")]
pub struct Cli {
    /// Team
    #[clap(short)]
    team: Vec<String>,
    /// Number of Fixtures to print
    #[clap(short)]
    nfix: usize,
}

#[derive(Debug)]
pub struct UserOptions {
    pub team: TeamScope,
    pub nfix: usize,
    pub icon_style: String,
    pub date_format: String,
}

#[derive(clap::ValueEnum, Clone, Debug, Copy)]
enum TeamScope {
    Leinster,
    Munster,
    Connacht,
    Ulster,
}

fn get_team(param: String) -> Result<TeamScope, Box<dyn std::error::Error>> {
    let p_str = param.as_str();

    match p_str {
        "leinster" => return Ok(TeamScope::Leinster),
        "munster" => return Ok(TeamScope::Munster),
        "connacht" => return Ok(TeamScope::Connacht),
        "ulster" => return Ok(TeamScope::Ulster),
        _ => return Err("team is invalid".into()),
    }
}

fn get_url(team: TeamScope, date: NaiveDate) -> String {
    let team_str = match team {
        TeamScope::Leinster => "leinster",
        TeamScope::Munster => "munster",
        TeamScope::Connacht => "connacht",
        TeamScope::Ulster => "ulster",
    };

    let mut url = String::from(URL_BASE_BBC);
    url = url.replace("PLACEHOLDER", team_str);

    url.push_str(&date.format("%Y-%m").to_string());

    url
}

fn get_next_fixtures(
    n: usize,
    date: NaiveDate,
    team: TeamScope,
) -> Result<FixtureList, Box<dyn std::error::Error>> {
    let mut fxlist = FixtureList::new();

    let mut local_date = date.clone();

    loop {
        let url = get_url(team, local_date);
        let fx_res = get_flist(url, &mut fxlist, local_date.year());
        match fx_res {
            Ok(len) => {
                if len < n {
                    local_date = local_date.checked_add_months(Months::new(1)).unwrap();
                    local_date = local_date
                        .checked_sub_days(Days::new(local_date.day() as u64 - 1))
                        .unwrap();
                } else if len == n {
                    break;
                } else {
                    fxlist.truncate(n);
                    break;
                }
            }
            Err(e) => return Err(e),
        }
    }
    return Ok(fxlist);
}

fn build_user_options(cli: &Cli, config: Option<AppConfig>) -> Result<UserOptions, String> {
    let default_icon = "rugby".to_string();
    let default_date = "%Y-%m-%d".to_string();

    let (team, nfix) = {
        let cfg_pref = config.as_ref().and_then(|c| c.preferences.as_ref());

        let team = Some(cli.team.clone())
            .or_else(|| cfg_pref.and_then(|p| p.teams.clone()))
            .ok_or("Missing required field: team")?;

        let nfix = Some(cli.nfix.clone())
            .or_else(|| cfg_pref.and_then(|p| p.nfix))
            .ok_or("Missing required field: nfix")?;

        (team, nfix)
    };

    let (icon_style, date_format) = {
        let fmt = config.and_then(|c| c.formatting);
        (
            fmt.as_ref()
                .and_then(|f| f.icon_style.clone())
                .unwrap_or(default_icon),
            fmt.and_then(|f| f.date_format.clone())
                .unwrap_or(default_date),
        )
    };

    let team_enum = get_team(team[0].clone()).unwrap();

    Ok(UserOptions {
        team: team_enum,
        nfix,
        icon_style,
        date_format,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::try_parse();
    let config = AppConfig::try_load();

    let params = build_user_options(&cli.unwrap(), Some(config.unwrap()));

    let team = params.as_ref().unwrap().team;
    let n = params.unwrap().nfix;

    let today = Utc::now();

    let fxlist = match get_next_fixtures(n, today.date_naive(), team) {
        Ok(fxlist) => fxlist,
        Err(e) => panic!("error getting fixtures: {:?}", e),
    };

    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK");

    // Create the app indicator with an IndicatorCategory
    let indicator = Indicator::new(
        "rugby-fixtures",                                    // Indicator ID
        "indicator-messages",                                // Icon name
        appindicator3::IndicatorCategory::ApplicationStatus, // Category
    );

    indicator.set_icon_full("/home/charlie/.icons/rugby.jpg", "Rugby Icon");

    // Set the status of the indicator
    indicator.set_status(IndicatorStatus::Active);

    // Create the menu
    let menu = Menu::new();

    // Add fixtures to the menu
    for fixture in fxlist {
        let item = MenuItem::with_label(fixture.format_fixture().as_str());
        item.show();
        menu.append(&item);
    }

    // Add quit button
    let quit_item = MenuItem::with_label("Quit");
    quit_item.connect_activate(|_| gtk::main_quit());
    quit_item.show();
    menu.append(&quit_item);

    // Set the menu on the indicator
    indicator.set_menu(Some(&menu));

    // Background thread to update the indicator
    thread::spawn(move || {
        loop {
            // Normally, fetch new data here and update the menu
            thread::sleep(Duration::from_secs(3600)); // Refresh every hour
        }
    });

    // Run GTK main loop
    gtk::main();

    Ok(())
}
