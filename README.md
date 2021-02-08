# rssurl

A very simple command-line utility to extract links from RSS feeds,
for use in shell-scripts.

You can probably do this as a complex Perl one-liner. There's probably
more comprehensive tools out there which does the same thing.

I wanted to do this in rust, so here we are!

## features

- obtain links from ATOM-feeds
- nothing else

## building

````sh
git clone https://github.com/josteink/rssurl
cd rssurl
cargo build
````

## basic usage

To use rssurl, find the ATOM feed for a given site, and point it at
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
# step 1: obtain ATOM feed URL (view source, however)
FEED=https://www.youtube.com/feeds/videos.xml?channel_id=UCrTNhL_yO3tPTdQ5XgmmWjA
# step 2: build
cargo build
# invoke prebuilt binary directly to avoid cargo build-headers in output!
URLS=`./target/debug/rssurl $FEED`
# get it yer plex!
youtube-dl $URLS
````


