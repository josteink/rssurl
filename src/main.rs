extern crate reqwest;
extern crate syndication;
extern crate atom_syndication;
extern crate rss;

use structopt::StructOpt;
use std::error::Error;
use syndication::Feed;

#[derive(StructOpt, Debug)]
struct Cli {
    /// The path to the file to read
    url: String
}


fn main() {
    let args = Cli::from_args();

    let result = process(&args.url);
    match result {
        Ok(_) => {},
        Err(e) => println!("Oh noes. Something bad happened: {}", e),
    }
}

fn process(url: &str) -> Result<(),Box<dyn Error>> {
    let rss_str = get_text(url)?;
    let entries = get_entries(&rss_str)?;
    dump_entries(&entries);
    Ok(())
}

fn get_text(url: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?;
    let rss_str = response.text()?;
    Ok(rss_str.to_string())
}

fn get_entries(rss_str: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match rss_str.parse::<Feed>().unwrap() {
        Feed::RSS(rss_feed) => get_rss_entries(rss_feed),
        Feed::Atom(atom_feed) => get_atom_entries(atom_feed)
    }
}

fn get_rss_entries(rss_feed: rss::Channel) -> Result<Vec<String>, Box<dyn Error>> {
    let mut vec = Vec::new();

    for entry in rss_feed.items() {
        let url = entry.link.as_ref().unwrap().to_string();
        vec.push(String::from(url));
    }

    Ok(vec)
}

fn get_atom_entries(atom_feed: atom_syndication::Feed) -> Result<Vec<String>, Box<dyn Error>> {
    let mut vec = Vec::new();

    for entry in atom_feed.entries() {
        let link = &entry.links()[0];
        let url = link.href().to_string();

        vec.push(String::from(url));
    }

    Ok(vec)
}

fn dump_entries(entries: &Vec<String>) {
    for url in entries {
        println!("{}", url);
    }
}
