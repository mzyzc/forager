use curl::easy::Easy;
use roxmltree;
use std::io::{Error, ErrorKind};

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

pub fn add_feed(url: &str) -> Result<Vec<FeedItem>, Error> {
    let feed = fetch_feed(url)?;

    let feed_items = parse_feed(&feed);
    match feed_items {
        Ok(f) => return Ok(f),
        Err(e) => return Err(e),
    };
}

fn fetch_feed(url: &str) -> Result<String, Error> {
    let mut data = Vec::new();
    let mut handle = Easy::new();

    handle.url(url)?;
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        })?;
        transfer.perform()?;
    }

    Ok(String::from_utf8_lossy(&data).to_string())
}

fn parse_feed(feed: &str) -> Result<Vec<FeedItem>, Error> {
    let doc = match roxmltree::Document::parse(feed) {
        Ok(d) => d,
        Err(_) => return Err(Error::new(ErrorKind::InvalidData, "could not parse XML data")),
    };
    let mut pointer = doc.root_element();
    let mut feed_list = Vec::new();
    
    if !pointer.has_tag_name("rss") {
        return Err(Error::new(ErrorKind::InvalidData, "bad 'feed' node"));
    }

    pointer = match pointer.first_element_child() {
        Some(c) => c,
        None => return Err(Error::new(ErrorKind::InvalidData, "no child for 'feed' node")),
    };

    if !pointer.has_tag_name("channel") {
        return Err(Error::new(ErrorKind::InvalidData, "bad 'channel' node"));
    }

    let items = pointer.children()
        .filter(|node| node.has_tag_name("item"));

    for item in items {
        pointer = item;

        let mut title = String::new();
        let mut link = String::new();
        let mut description = String::new();

        if !pointer.has_tag_name("item") {
            return Err(Error::new(ErrorKind::InvalidData, "bad 'item' node"));
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

    Ok(feed_list)
}
