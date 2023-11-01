use headless_chrome::Browser;
use headless_chrome::protocol::cdp::Page;
use std::error::Error;
use std::io::{self, Write};

fn google_search(search: &str) -> Result<(), Box<dyn Error>> {
    // Google search code
    // todo: get the browser to open lol

    let browser = Browser::default()?;

    let tab = browser.new_tab()?;

    let query = search;
    println!("this is the query you are making: {}", query);

    let url = format!("https://www.google.co.in/search?q={}", query);
    tab.navigate_to(&url)?;

    let _jpeg_data = tab.capture_screenshot(
        Page::CaptureScreenshotFormatOption::Jpeg,
        None,
        None, 
        true)?;

    Ok(())
}

fn input_data() -> String {
    println!("enter what you want to search today?");
    // flush() pushes all outputs out of the stack to make output forcefully appear
    io::stdout().flush().unwrap();

    let mut search: String = String::new();

    // input reader
    io::stdin()
        .read_line(&mut search)
        .expect("there is an issue when readint the line!");

    // cleaning data off whitespaces and making it into a string for safety measure
    let data_cleaner: String = search.trim().to_string();

    // setting string limits for efficient speeds
    if data_cleaner.len() > 20 {
        println!("please reduce the search size, be concise as much as possible");
        return input_data();
    } else {
        // println!("{}", data_cleaner);
        return data_cleaner;
    }
}

fn info() {
    println!("welcome to the cli web shooter!");
    println!("since this is patch 1.0, we will be mostly using Chrome for our searches!");
}

fn main() {
    // info() to print out all the info when this starts
    // cuz we in Beta and there will be information that needs to be displayed
    info();
    // function dedicted to get the data from the user on what to search
    let search_results = input_data();

    // actual function to search for the data
    google_search(&search_results);
}
