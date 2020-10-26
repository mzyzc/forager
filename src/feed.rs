use curl::easy::Easy;

pub fn print_feed() {
    let xml = fetch_xml("https://news.ycombinator.com/rss");
    println!("{}", xml);
}

fn fetch_xml(url: &str) -> String {
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

// struct feedItem: title, link, description

// function parse_rss(string) -> [feedItem]

// function print_feed(feedItem)
