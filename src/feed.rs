use curl::easy::Easy;
use roxmltree;

pub struct FeedItem {
    pub title: String,
    pub link: String,
    pub description: String,
}

pub fn print_feed() {
    let feed = fetch_feed("https://news.ycombinator.com/rss");
    println!("Fetch successful");
    parse_feed(&feed);
}

// Fetch XML data from a URL
fn fetch_feed(url: &str) -> String {
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

// function parse_feed(string) -> [feedItem]
fn parse_feed(feed: &str) {
    let doc = roxmltree::Document::parse(feed).unwrap();
    let mut pointer = doc.root_element();
    
    if !pointer.has_tag_name("rss") {
        panic!("invalid feed: bad 'feed' node")
    }

    pointer = pointer.first_element_child().expect("invalid feed: no child for 'feed' node");

    if !pointer.has_tag_name("channel") {
        panic!("invalid feed: bad 'channel' node")
    }

    let items = pointer.children()
        .filter(|node| node.has_tag_name("item"));

    for item in items {
        pointer = item;

        if !pointer.has_tag_name("item") {
            panic!("invalid feed: bad 'item' node")
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
