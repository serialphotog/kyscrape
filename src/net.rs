use select::document::Document;
use select::predicate::{Class, Name, Predicate};

use crate::landform;

/// Downloads data from a given URL
fn download_page(url: &str) -> reqwest::blocking::Response {
    let response = reqwest::blocking::get(url).unwrap();
    assert!(response.status().is_success());
    response
}

/// Builds a URL from a base and a part (concats the two)
pub fn build_url(base_url: &str, part: String) -> String {
    let mut base = base_url.to_string();
    base.push_str(&part);
    base
}

/// Gets the various landform containing pages from the database
pub fn get_database_pages(url: &str, output: &mut Vec<String>, target: String,
    exclusions: Option<&Vec<String>>) {
    let data = download_page(url);
    let document = Document::from_read(data).unwrap();

    for node in document.find(Class("module-inner")) {
        let module_title = node.find(Class("module-title").descendant(Name("span")))
            .next()
            .unwrap();
        if module_title.text() == target {
            node.find(Name("a"))
                .filter_map(|n| n.attr("href"))
                .for_each(|x|
                // Exclusions
                if !exclusions.is_none() {
                    if !exclusions.unwrap().iter().any(|i| i == &x.to_string()) {
                        output.push(x.to_string());
                    }
                } else {
                    output.push(x.to_string());
                }
            ); 
        }
    }
}

/// Gets links to all the landforms in the database
pub fn get_landforms(url: &str, output: &mut Vec<String>) {
    let data = download_page(url);
    let document = Document::from_read(data).unwrap();

    for node in document.find(Class("category")) {
        node.find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .for_each(|x| if x != "#" {
                output.push(x.to_string())
            });
    }
}

/// Downloads an individual landform from the database
pub fn download_landform(url: String) -> landform::Landform {
    let data = download_page(&url);
    let document = Document::from_read(data).unwrap();

    let mut name: String = "".to_owned();
    let mut lat: String = "".to_owned();
    let mut lon: String = "".to_owned();

    for node in document.find(Class("article-content")) {
        // Search for the table rows
        node.find(Name("tr"))
            .for_each(|x| x.find(Name("td")).for_each(
                |i| if i.text() == "Name" {
                    name = i.next().unwrap().text(); 
                } else if i.text() == "Latitude" {
                    lat = i.next().unwrap().text();
                } else if i.text() == "Longitude" {
                    lon = i.next().unwrap().text();
                }
            ));
    }

    let item = landform::Landform {name: name, latitude: lat, longitude: lon};
    item
}