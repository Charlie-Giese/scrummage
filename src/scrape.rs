use scraper::{Html, Selector, ElementRef};
use crate::fixtures::{Fixture, FixtureList, Teams}; 

use std::iter::zip;

 fn get_html_document(url : String) -> Html {
    
    println!("{:?}", url);

    let response = reqwest::blocking::get(url);
    let html_content = response.unwrap().text().unwrap();

    let parsed_html = Html::parse_document(&html_content);

    parsed_html
}

fn get_child_nodes(parsed_html : &Html) -> Option<ElementRef> {
    
   
    let main_selector = &Selector::parse("div#main-data").unwrap();
    let main_body = parsed_html
        .select(&main_selector)
        .next()
        .unwrap();
    let child_node_selector = &Selector::parse("div").unwrap();
    let children = main_body
        .select(&child_node_selector)
        .next();

    children
}

fn populate_flist(team : String, children : ElementRef) -> FixtureList {

    let dates = children
        .select(&Selector::parse("h2").unwrap())
        .map(|elem| elem.text().next().unwrap().to_string())
        .collect::<Vec<_>>();

     let comps = children
        .select(&Selector::parse("h3").unwrap())
        .map(|elem| elem.text().next().unwrap().to_string())
        .collect::<Vec<_>>();

     let times = children
        .select(&Selector::parse("time").unwrap())
        .map(|elem| elem.text().next().unwrap().to_string())
        .collect::<Vec<_>>();

    let teams = children
        .select(&Selector::parse("span").unwrap())
        .filter(|elem| elem.value().classes().next().unwrap() == "emlpoi30")
        .collect::<Vec<_>>();

    let mut flist = FixtureList::new(times.len());
    
    for (i, (date, (time, comp))) in zip(dates, zip(times, comps)).enumerate() {
        let mut current = Teams::new();
        current.home = teams[2 * i].text().next().unwrap().to_string();
        current.away = teams[2 * i + 1].text().next().unwrap().to_string();
        let fx = Fixture::new(current, date, time, comp);
        flist.push_fx(fx);
    }

    flist
}

pub fn get_flist(team : String, url : String) -> FixtureList {

    let document = get_html_document(url);
    let child_nodes = match get_child_nodes(&document) {
        Some(x)     => x,
        None        => panic!("error getting child nodes..."),
    };

    let flist = populate_flist(team, child_nodes);

    flist

}

