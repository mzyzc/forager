use curl::easy::Easy;
use roxmltree;

pub struct FeedItem {
    pub title: String,
    pub link: String,
    pub description: String,
}

// Fetch XML data from a URL
pub fn print_feed() {
    let rss = fetch_rss("https://news.ycombinator.com/rss");
    println!("Fetch successful");
    parse_rss(&rss);
}

fn fetch_rss(url: &str) -> String {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(url).unwrap();
    {

        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    String::from_utf8_lossy(&data).to_string()
}

// function parse_rss(string) -> [feedItem]
fn parse_rss(rss: &str) {
    let doc = roxmltree::Document::parse(rss).unwrap();
    let mut pointer = doc.root_element();
    
    if !pointer.has_tag_name("rss") {
        panic!("invalid RSS feed: bad 'rss' node")
    }

    pointer = pointer.first_element_child().expect("invalid RSS feed: no child for 'rss' node");

    if !pointer.has_tag_name("channel") {
        panic!("invalid RSS feed: bad 'channel' node")
    }

    let items = pointer.children()
        .filter(|node| node.has_tag_name("item"));

    for item in items {
        pointer = item;

        if !pointer.has_tag_name("item") {
            panic!("invalid RSS feed: bad 'item' node")
        }

        for element in item.children() {
            pointer = element;

            match pointer.tag_name().name() {
                "title" => println!("title: {}", pointer.text().unwrap()),
                "link" => println!("link: {}", pointer.text().unwrap()),
                "description" => println!("description: {}", pointer.text().unwrap()),
                _ => (),
            }
        }

        println!();
    }
}
