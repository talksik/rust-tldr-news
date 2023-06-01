use std::io::Read;

use chrono::prelude::*;
use colored::*;

const TLDR_BASE_URL: &str = "https://tldr.tech/tech";

fn retrieve_tldr_html(date_string: &str) -> String {
    let complete_url = format!("{}{}", TLDR_BASE_URL, date_string);

    reqwest::blocking::get(complete_url)
        .unwrap()
        .text()
        .unwrap()
}

// takes a file name and returns the contents of the file as a string
fn retrieve_local_file(file_name: &str) -> String {
    println!("reading file {}", file_name);

    let mut file = std::fs::File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
}

fn main() {
    println!("hello world");

    // format to 2023-06-05
    let today = Local::now();
    let formatted_date = today.format("%Y-%m-%d");
    println!("fetching tldr for {}", formatted_date);

    //let webpage_contents = retrieve_tldr_html(&formatted_date.to_string());
    let webpage_contents = retrieve_local_file(&formatted_date.to_string());

    let document = scraper::Html::parse_document(&webpage_contents);
    let selector = scraper::Selector::parse("div.mt-3").unwrap();

    for element in document.select(&selector) {
        // get nested h3
        let title_selector = scraper::Selector::parse("h3").unwrap();
        let title_element = match element.select(&title_selector).next() {
            Some(element) => element,
            None => continue,
        };

        // inner div contains the description
        let description_selector = scraper::Selector::parse("div").unwrap();
        let description_element = element.select(&description_selector).next().unwrap();

        println!("==================");
        println!("{}", title_element.inner_html().green());
        println!("\n");
        println!("{}", description_element.inner_html());
        println!("\n");
    }
}
