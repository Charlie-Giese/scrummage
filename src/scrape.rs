use scraper::{Html, Selector, ElementRef};
use itertools::Itertools;

struct Fixture {
    date : String,
    opponent : String,
    competition : String,
}

struct FixtureList {
    length : usize,
    fixtures : Vec<Fixture>,
}

struct FixtureScraper {
    fixtures        : FixtureList,
    html_doc        : Html,
    html_body       : ElementRef,
}

impl FixtureScraper {
    
    fn get_html_document(&self, url : &str) -> Html {
        
        let response = reqwest::blocking::get(url);
        let html_content = response.unwrap().text().unwrap();

        let parsed_html = Html::parse_document(&html_content);





    fn populate_fixtures(&self, html_fixture_body : ElementRef) {

        let dates = children
            .select(&Selector::parse("h2").unwrap())
            .map(|elem| elem.text().next().unwrap())
            .collect::<Vec<_>>();

         let comps = children
            .select(&Selector::parse("h3").unwrap())
            .map(|elem| elem.text().next().unwrap())
            .collect::<Vec<_>>();

         let times = children
            .select(&Selector::parse("time").unwrap())
            .map(|elem| elem.text().next().unwrap())
            .collect::<Vec<_>>();

        let team_spans = children
            .select(&Selector::parse("span").unwrap())
            .collect::<Vec<_>>();

        let mut teams : Vec<&str> = vec![];

        for span in team_spans {
            match span.value().classes().next().unwrap() {
                "emlpoi30"      => teams.push(span.text().next().unwrap()),
                &_              => (),
            };
        }
    }
}
pub fn get_fixture_body(url : &str) -> Option<ElementRef> {
    
   
    let main_selector = &Selector::parse("div#main-data").unwrap();
    let fixture_body = parsed_html
        .select(&main_selector)
        .next();
    let child_selector = &Selector::parse("div").unwrap();
    let children = fixture_body
        .select(&child_selector)
        .next()
        .unwrap();

    children
}


