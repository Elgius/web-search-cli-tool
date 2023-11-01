use headless_chrome::Browser;
use std::io::{self, Write};
// use std::error::Error as StdError;
use failure::Error;

fn google_search(search: &str) -> Result<(), failure::Error> {
    // Google search code, work in progress

    let browser = Browser::default().map_err(Error::from_boxed_compat(err))?;

    let tab = browser.new_tab().map_err(Error::from_boxed_compat(err))?;

    let url = "https://www.google.co.in/search?q={search}";
    tab.navigate_to(&url).map_err(Error::from_boxed_compat(err))?;

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
        println!("{}", data_cleaner);
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
