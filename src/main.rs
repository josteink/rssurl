extern crate reqwest;
extern crate syndication;
extern crate atom_syndication;
extern crate rss;

use structopt::StructOpt;
use std::error::Error;
use syndication::Feed;

#[derive(StructOpt, Debug)]
struct Cli {
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rss_detected() {
        let rss_str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>neko-net irc</title>
    <description>The neko-net IRC network's main homepage with news, servers and more.
</description>
    <link>http://www.neko-net.org/</link>
    <atom:link href="http://www.neko-net.org/feed.xml" rel="self" type="application/rss+xml"/>
    <pubDate>Fri, 25 Oct 2019 11:34:07 +0000</pubDate>
    <lastBuildDate>Fri, 25 Oct 2019 11:34:07 +0000</lastBuildDate>
      <item>
        <title>Server Downtime</title>
        <description>&lt;p&gt;After some server-maintenance last evening, our Oslo-server is currently down.
There are both issues with SSL and with routing, and both are being worked on.&lt;/p&gt;
&lt;p&gt;&lt;strong&gt;Update:&lt;/strong&gt; All services are now back up.&lt;/p&gt;
</description>
        <pubDate>Thu, 24 Oct 2019 06:30:00 +0000</pubDate>
        <link>http://www.neko-net.org/server/2019/10/24/server-downtime.html</link>
        <guid isPermaLink="true">http://www.neko-net.org/server/2019/10/24/server-downtime.html</guid>
        <category>server</category>
      </item>
      <item>
        <title>Expired SSL certificates</title>
        <description>&lt;p&gt;Due to an incomplete DNS-migration from GoDaddy to Cloudflare, all servers in the network
were unable to retrieve updated certificates.&lt;/p&gt;
&lt;p&gt;This issue is now resolved, and servers should be able to update their certificatates again.&lt;/p&gt;
&lt;p&gt;Sorry about the inconvenience.&lt;/p&gt;
</description>
        <pubDate>Mon, 30 Jul 2018 10:00:00 +0000</pubDate>
        <link>http://www.neko-net.org/server/2018/07/30/expired-ssl-certificates.html</link>
        <guid isPermaLink="true">http://www.neko-net.org/server/2018/07/30/expired-ssl-certificates.html</guid>
        <category>server</category>
      </item>
  </channel>
</rss>
"#;

        let entries = get_entries(rss_str).unwrap();
        let num_entries = entries.len();

        assert_eq!(num_entries, 2);
        assert_eq!(entries[0], "http://www.neko-net.org/server/2019/10/24/server-downtime.html");
        assert_eq!(entries[1], "http://www.neko-net.org/server/2018/07/30/expired-ssl-certificates.html");
    }

    #[test]
    fn atom_detected() {
        let atom_str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom" xmlns:media="http://search.yahoo.com/mrss/">
  <category term=" reddit.com" label="r/ reddit.com"/>
  <updated>2021-02-09T07:01:20+00:00</updated>
  <id>/.rss</id>
  <link rel="self" href="https://www.reddit.com/.rss" type="application/atom+xml" />
  <link rel="alternate" href="https://www.reddit.com/" type="text/html" />
  <title>reddit: the front page of the internet</title>
  <entry>
    <author>
      <name>/u/PR0CR45T184T0R</name>
      <uri>https://www.reddit.com/user/PR0CR45T184T0R</uri>
    </author>
    <category term="pics" label="r/pics"/>
    <content type="html">&lt;table&gt; &lt;tr&gt;&lt;td&gt; &lt;a href=&quot;https://www.reddit.com/r/pics/comments/lfrv24/jack_black_walks_past_a_couple_and_unknowingly/&quot;&gt; &lt;img src=&quot;https://a.thumbs.redditmedia.com/MPMCBwn51e7T6PbwLflO1GBW3t0EI1cnfrPQlcOQTH0.jpg&quot; alt=&quot;Jack Black walks past a couple and unknowingly recreates the &amp;quot;Distracted Boyfriend&amp;quot; meme&quot; title=&quot;Jack Black walks past a couple and unknowingly recreates the &amp;quot;Distracted Boyfriend&amp;quot; meme&quot; /&gt; &lt;/a&gt; &lt;/td&gt;&lt;td&gt; &amp;#32; submitted by &amp;#32; &lt;a href=&quot;https://www.reddit.com/user/PR0CR45T184T0R&quot;&gt; /u/PR0CR45T184T0R &lt;/a&gt; &amp;#32; to &amp;#32; &lt;a href=&quot;https://www.reddit.com/r/pics/&quot;&gt; r/pics &lt;/a&gt; &lt;br/&gt; &lt;span&gt;&lt;a href=&quot;https://i.redd.it/owife97izcg61.png&quot;&gt;[link]&lt;/a&gt;&lt;/span&gt; &amp;#32; &lt;span&gt;&lt;a href=&quot;https://www.reddit.com/r/pics/comments/lfrv24/jack_black_walks_past_a_couple_and_unknowingly/&quot;&gt;[comments]&lt;/a&gt;&lt;/span&gt; &lt;/td&gt;&lt;/tr&gt;&lt;/table&gt;</content>
    <id>t3_lfrv24</id>
    <media:thumbnail url="https://a.thumbs.redditmedia.com/MPMCBwn51e7T6PbwLflO1GBW3t0EI1cnfrPQlcOQTH0.jpg" />
    <link href="https://www.reddit.com/r/pics/comments/lfrv24/jack_black_walks_past_a_couple_and_unknowingly/" />
    <updated>2021-02-09T01:59:02+00:00</updated>
    <title>Jack Black walks past a couple and unknowingly recreates the &quot;Distracted Boyfriend&quot; meme</title>
  </entry>
  <entry>
    <author>
      <name>/u/nolessdays</name>
      <uri>https://www.reddit.com/user/nolessdays</uri>
    </author>
    <category term="books" label="r/books"/>
    <content type="html">&lt;!-- SC_OFF --&gt;&lt;div class=&quot;md&quot;&gt;&lt;p&gt;I’ll go first. I prefer my Kindle to the point that I’ll download a book on my Kindle even if we already own the hard copy. A Little Life was ok but not great and did not make me cry. I did not finish Circe by Madeline Miller because it was boring. &lt;/p&gt; &lt;p&gt;How about you? What are your unpopular book opinions?&lt;/p&gt; &lt;/div&gt;&lt;!-- SC_ON --&gt; &amp;#32; submitted by &amp;#32; &lt;a href=&quot;https://www.reddit.com/user/nolessdays&quot;&gt; /u/nolessdays &lt;/a&gt; &amp;#32; to &amp;#32; &lt;a href=&quot;https://www.reddit.com/r/books/&quot;&gt; r/books &lt;/a&gt; &lt;br/&gt; &lt;span&gt;&lt;a href=&quot;https://www.reddit.com/r/books/comments/lfctle/what_are_your_most_unpopular_book_opinions/&quot;&gt;[link]&lt;/a&gt;&lt;/span&gt; &amp;#32; &lt;span&gt;&lt;a href=&quot;https://www.reddit.com/r/books/comments/lfctle/what_are_your_most_unpopular_book_opinions/&quot;&gt;[comments]&lt;/a&gt;&lt;/span&gt;</content>
    <id>t3_lfctle</id>
    <link href="https://www.reddit.com/r/books/comments/lfctle/what_are_your_most_unpopular_book_opinions/" />
    <updated>2021-02-08T14:24:08+00:00</updated>
    <title>What are your most unpopular book opinions?</title>
  </entry>
</feed>
"#;

        let entries = get_entries(atom_str).unwrap();
        let num_entries = entries.len();

        assert_eq!(num_entries, 2);
        assert_eq!(entries[0], "https://www.reddit.com/r/pics/comments/lfrv24/jack_black_walks_past_a_couple_and_unknowingly/");
        assert_eq!(entries[1], "https://www.reddit.com/r/books/comments/lfctle/what_are_your_most_unpopular_book_opinions/");
    }
}
