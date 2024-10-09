mod scrape;

use scrape::get_html_tree;

fn main() {

    let url = "https://www.bbc.com/sport/rugby-union/teams/leinster/scores-fixtures";

    get_html_tree(url);

}
