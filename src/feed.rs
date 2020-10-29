use curl::easy::Easy;
use roxmltree;

pub struct FeedItem {
    pub title: String,
    pub link: String,
    pub description: String,
}

impl FeedItem {
    pub fn clone(&self) -> FeedItem {
        let title = self.title.clone();
        let link = self.link.clone();
        let description = self.description.clone();
        FeedItem { title: title, link: link, description: description }       
    }
}

// Read data from a feed stored online
pub fn add_feed(url: &str) -> Vec<FeedItem> {
    let feed = fetch_feed(url);
    println!("Fetch successful");

    let feed_items = parse_feed(&feed);
    println!("Parse successful");
    println!();

    feed_items
}

// Fetch data from a URL
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

// Extract feed items from an XML file
fn parse_feed(feed: &str) -> Vec<FeedItem> {
    let doc = roxmltree::Document::parse(feed).unwrap();
    let mut pointer = doc.root_element();
    let mut feed_list = Vec::new();
    
    if !pointer.has_tag_name("rss") {
        panic!("invalid feed: bad 'feed' node");
    }

    pointer = pointer.first_element_child().expect("invalid feed: no child for 'feed' node");

    if !pointer.has_tag_name("channel") {
        panic!("invalid feed: bad 'channel' node");
    }

    let items = pointer.children()
        .filter(|node| node.has_tag_name("item"));

    for item in items {
        pointer = item;

        let mut title = String::new();
        let mut link = String::new();
        let mut description = String::new();

        if !pointer.has_tag_name("item") {
            panic!("invalid feed: bad 'item' node");
        }

        for element in item.children() {
            pointer = element;

            match pointer.tag_name().name() {
                "title" => { title = pointer.text().unwrap().to_string() }
                "link" => { link = pointer.text().unwrap().to_string() }
                "description" => { description = pointer.text().unwrap().to_string() }
                _ => (),
            }
        }

        feed_list.push(FeedItem { title: title, link: link, description: description });
    }

    feed_list
}
