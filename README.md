# rssurl

![Rust](https://github.com/josteink/rssurl/workflows/Rust/badge.svg)

A very simple command-line utility to extract links from RSS feeds,
for use in shell-scripts.

You can probably do this as a complex Perl one-liner. There's probably
more comprehensive tools out there which does the same thing.

I wanted to do this in rust, so here we are!

## features

- obtain links from ATOM-feeds and RSS-feeds
- automatically detect feed-type
- statically linked, because rust, all the way, including the crypto-dependencies

## building

````sh
git clone https://github.com/josteink/rssurl
cd rssurl
cargo build
````

## basic usage

To use rssurl, find the feed-url for a given site, and point it at
this tool.

````sh
# list current trending reddit links
cargo run https://www.reddit.com/.rss
````

## more involved example

Want to grab the latest [Red Letter Media](https://www.youtube.com/user/RedLetterMedia)
shows and import then into your Plex?

Compose! Go crazy! Make it happen!

````sh
# step 1: obtain feed-url (view source, however)
FEED=https://www.youtube.com/feeds/videos.xml?channel_id=UCrTNhL_yO3tPTdQ5XgmmWjA
# step 2: build
cargo build
# invoke prebuilt binary directly to avoid cargo build-headers in output!
URLS=`./target/debug/rssurl $FEED`
# get it on yer plex!
youtube-dl $URLS
````


