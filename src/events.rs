extern crate gtk;

use crate::feed;
use gtk::prelude::*;
use gtk::{ListBox, ListBoxRow, Label};

pub fn update_list(list: &gtk::ListBox, url: &str) {
    let items = feed::add_feed(url);

    for item in items.iter() {
        let row = ListBoxRow::new();

        let label = Label::new(Some(&item.title));
        label.set_xalign(0.0);
        row.add(&label);

        list.prepend(&row);
        list.show_all();

        println!("title: {}", item.title);
        println!("link: {}", item.link);
        println!("description: {}", item.description);
        println!();
    }
}
