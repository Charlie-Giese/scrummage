mod scrape;
mod fixtures;

use scrape::get_flist;

fn main() {

    let url = "https://www.bbc.com/sport/rugby-union/teams/leinster/scores-fixtures";

    let flist = get_flist(String::from("Leinster"), url);

    flist.print_flist();

}
