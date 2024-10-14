mod scrape;
mod fixtures;

use scrape::get_flist;
use clap::{Parser};

const URL_BASE_BBC : &str = "https://www.bbc.com/sport/rugby-union/teams/PLACEHOLDER/scores-fixtures";


#[derive(Parser, Debug)]
#[clap(author="Charles Giese", version, about="Get Rugby Fixtures")]
pub struct Cli {
    /// Team
    #[clap(short)]
    team : TeamScope,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum TeamScope {
    Leinster,
    Munster,
    Connacht,
    Ulster,
    Ireland,
}

fn get_url(team: TeamScope) -> String {
    
    let team_str = match team {
        TeamScope::Leinster     => "leinster",
        TeamScope::Munster      => "munster",
        TeamScope::Connacht     => "connacht",
        TeamScope::Ulster       => "ulster",
        TeamScope::Ireland      => "ireland",
    };

    let url = URL_BASE_BBC;
    url.replace("PLACEHOLDER", team_str)

}

fn main() {

    let team = Cli::parse().team;
    let url = get_url(team);

    let flist = get_flist(String::from("Leinster"), url);

    flist.print_flist();

}

