use crate::feed;
use gtk::prelude::*;
use gtk::{ListBoxRow, Label};

// Import feed into a GTK list
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

// Update the preview when a feed item is selected
pub fn update_preview(preview: &gtk::Box, row: &gtk::ListBoxRow) {
    let title = Label::new(None);
    title.set_markup("<big>this is the title</big>");
    let description = Label::new(Some("this is the description"));
    let link = Label::new(None);
    link.set_markup("<u>this is a link</u>");

    // Remove old preview elements
    let preview_items = preview.get_children();
    for item in preview_items.iter() {
        preview.remove(item);
    }

    preview.add(&title);
    preview.add(&description);
    preview.add(&link);

    preview.show_all();
}
