// use std::error::Error;
use std::io::{self, Write};
// use std::path::PathBuf;
// use std::io::Error;
use std::process::Command;
// use std::str::Utf8Error;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

fn mozilla_firefox(query: String) {
    // Note, this only supports single word arguments at the moment!
    let queries: String = format!(
        "start microsoft-edge:https://www.bing.com/search?q={}",
        query
    )
    .trim()
    .parse()
    .expect("Query issue has occured");
    let _output = Command::new("cmd")
        .arg("/C")
        .arg(queries)
        .output()
        .expect("Failed to execute command");
}

fn url_encoding(search: String) -> String {
    // check package docs on why we use fragment and need this for initializing
    const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'>').add(b'`');

    // this where we import user search thing, then encode it and export it
    let input = search;
    let iter = utf8_percent_encode(&input, FRAGMENT);
    let encode: String = iter.collect();
    encode
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
    if data_cleaner.len() > 50 {
        println!("please reduce the search size, be concise as much as possible");
        return input_data();
    } else {
        // println!("{}", data_cleaner);
        return data_cleaner;
    }
}

fn info() {
    println!("welcome to the cli web shooter!");
    println!("since this is patch 1.0, we will be mostly using Edge for our searches!");
}

fn main() {
    // info() to print out all the info when this starts
    // cuz we in Beta and there will be information that needs to be displayed
    info();
    // function dedicted to get the data from the user on what to search
    let search_results = input_data();

    // encoding the search data into URL format
    let story = url_encoding(search_results);

    println!("{}", story);
    // actual function to search for the data
    let _query = mozilla_firefox(story);
}
