extern crate reqwest;
extern crate atom_syndication;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    /// The path to the file to read
    url: String
}


use std::error::Error;
use atom_syndication::Feed;

fn main() {
    let args = Cli::from_args();

    let result = process(&args.url);
    match result {
        Ok(_) => {},
        Err(e) => println!("Oh noes. Something bad happened: {}", e),
    }
}

fn process(url: &str) -> Result<(),Box<dyn Error>> {
    let entries = get_entries(url)?;
    dump_entries(&entries);
    Ok(())
}

fn get_entries(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?;

    let text = response.text()?;

    let channel = text.parse::<Feed>();
    let channel = match channel {
        Ok(channel) => channel,
        Err(e) => panic!("Something went wrong: {}", e),
    };

    let mut vec = Vec::new();

    for entry in channel.entries() {
        let link = &entry.links()[0];
        let url = link.href().to_string();

        vec.push(url);
    }

    Ok(vec)
}

fn dump_entries(entries: &Vec<String>) {
    for url in entries {
        println!("{}", url);
    }
}
