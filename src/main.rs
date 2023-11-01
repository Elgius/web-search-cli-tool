use std::io::{self, Write};

fn input_data() -> String {
    println!("enter what you want to search today?");
    io::stdout().flush().unwrap();

    let mut search: String = String::new();

    io::stdin()
        .read_line(&mut search)
        .expect("there is an issue when readint the line!");

    let data_cleaner: String = search.trim().to_string();

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
    info();
    input_data();
}
