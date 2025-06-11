mod config;
mod datetime;
mod fixtures;
mod scrape;

use chrono::{Datelike, Days, Months, NaiveDate, Utc};
use config::Config;
use fixtures::FixtureList;
use scrape::get_flist;
use std::cmp::max;
use std::fmt;

use appindicator3::prelude::AppIndicatorExt;
use appindicator3::Indicator;
use appindicator3::IndicatorStatus;
use gtk::prelude::*;
use gtk::{Menu, MenuItem};

const URL_BASE_BBC: &str =
    "https://www.bbc.com/sport/rugby-union/teams/PLACEHOLDER/scores-fixtures/";

#[derive(Debug)]
pub struct UserOptions {
    pub team: TeamScope,
    pub nfix: usize,
    pub icon_style: String,
    pub date_format: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum TeamScope {
    Leinster,
    Munster,
    Connacht,
    Ulster,
    Ireland,
}

impl fmt::Display for TeamScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TeamScope::Leinster => "leinster",
            TeamScope::Munster => "munster",
            TeamScope::Connacht => "connacht",
            TeamScope::Ulster => "ulster",
            TeamScope::Ireland => "ireland",
        };
        write!(f, "{}", s)
    }
}

fn get_url(team: String, date: NaiveDate) -> String {
    let mut url = String::from(URL_BASE_BBC);
    url = url.replace("PLACEHOLDER", &team);

    url.push_str(&date.format("%Y-%m").to_string());

    url
}

fn fetch_fixtures(config: &Config) -> Result<FixtureList, Box<dyn std::error::Error>> {
    let mut fxlist = FixtureList::new();
    let mut date = Utc::now().date_naive();
    let per_team_fetch_count = max(
        config.match_count * 2 / config.teams.len(),
        config.match_count,
    );

    for team in config.teams.clone() {
        loop {
            let url = get_url(team.to_string(), date);
            let fx_res = get_flist(url, &mut fxlist, date.year());
            match fx_res {
                Ok(len) => {
                    if len < per_team_fetch_count {
                        date = date.checked_add_months(Months::new(1)).unwrap();
                        date = date
                            .checked_sub_days(Days::new(date.day() as u64 - 1))
                            .unwrap();
                    } else {
                        break;
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }

    fxlist.sort_by_date();
    fxlist.truncate(config.match_count);

    return Ok(fxlist);
}

fn run_tray_ui(fxlist: FixtureList, params: &Config) {
    gtk::init().expect("Failed to initialize GTK");

    let indicator = Indicator::new(
        "rugby-fixtures",
        "indicator-messages",
        appindicator3::IndicatorCategory::ApplicationStatus,
    );

    indicator.set_icon_full("/home/charlie/.icons/rugby.jpg", "Rugby Icon");
    indicator.set_status(IndicatorStatus::Active);

    let menu = Menu::new();

    for fixture in fxlist {
        let item = MenuItem::with_label(&fixture.format_fixture());
        item.show();
        menu.append(&item);
    }

    let quit_item = MenuItem::with_label("Quit");
    quit_item.connect_activate(|_| gtk::main_quit());
    quit_item.show();
    menu.append(&quit_item);

    indicator.set_menu(Some(&menu));

    gtk::main();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load the configuration file (falling back to default with a warning)
    let config = Config::load_config();

    // 2. Initialize logging (optional, but useful)
    //init_logging(&config);

    // 3. Show initial fixtures in tray (perhaps via AppIndicator)
    let fixtures = fetch_fixtures(&config)?;
    run_tray_ui(fixtures, &config);

    // 4. If config.refresh_interval > 0, enter periodic refresh loop
    //if (config.refresh == config::RefreshInterval::BootOnly) {
    //    let mut interval = tokio::time::interval(Duration::from_secs(interval * 60));
    //    loop {
    //        interval.tick().await;
    //        let fixtures = fetch_fixtures(config)?;
    //        run_tray_ui(fixtures, &config);
    //    }
    //}

    // 5. If refresh_interval is None or 0, just exit
    Ok(())
}
