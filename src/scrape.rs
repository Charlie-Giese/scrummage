use scraper::{Html, Selector, ElementRef};
use crate::fixtures::{Fixture, FixtureList, Teams}; 

use std::iter::zip;
use crate::datetime::format_datetimes;

fn get_html_document(url : String) -> Html {
    
    let response = reqwest::blocking::get(url).expect("could not access url...");
    let html_content = response.text().unwrap();

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

fn populate_flist(children : ElementRef, fxlist : &mut FixtureList, year : i32) -> Result<usize, Box<dyn std::error::Error>> {

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
    let datetimes = format_datetimes(times.clone(), dates.clone(), year);
    
    for (i, (datetime, comp)) in zip(datetimes, comps).enumerate() {
        let mut current = Teams::new();
        current.home = teams[2 * i].text().next().unwrap().to_string();
        current.away = teams[2 * i + 1].text().next().unwrap().to_string();
        let fx = Fixture::new(current, datetime, comp);
        fxlist.push_fx(fx);

    }

    return Ok(fxlist.get_len());
}

pub fn get_flist(url : String, fxlist : &mut FixtureList, year : i32) -> Result<usize, Box<dyn std::error::Error>> {

    let document = get_html_document(url);
    let child_nodes = match get_child_nodes(&document) {
        Some(x)     => x,
        None        => panic!("error getting child nodes..."),
    };

    let res = populate_flist(child_nodes, fxlist, year);

    return res;

}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;
    use scraper::{Html, Selector};

    #[test]
    fn test_get_html_document() {
        // Mock URL and response
        let mock_url = &mockito::server_url();
        let mock_html = "<html><body><h1>Test</h1></body></html>";

        // Set up the mock server response
        let _m = mock("GET", "/")
            .with_status(200)
            .with_header("content-type", "text/html")
            .with_body(mock_html)
            .create();

        // Call the function with the mock URL
        let result = get_html_document(mock_url.to_string());

        // Verify the HTML is parsed correctly
        assert!(result.root_element().select(&scraper::Selector::parse("h1").unwrap()).next().is_some());
    }

    #[test]
    fn test_get_child_nodes() {
        // Mock HTML content
        let mock_html = r#"
            <html>
                <body>
                    <div id="main-data">
                        <div id="child1">Content 1</div>
                        <div id="child2">Content 2</div>
                    </div>
                </body>
            </html>
        "#;

        // Parse the HTML content
        let parsed_html = Html::parse_document(mock_html);

        // Call the function to get the child node
        let result = get_child_nodes(&parsed_html);

        // Verify that the result is Some and matches the expected element
        assert!(result.is_some());

        // Verify the id of the first child node is "child1"
        let first_child = result.unwrap();
        let child_id = first_child.value().attr("id").unwrap();
        assert_eq!(child_id, "child1");
    }

}

