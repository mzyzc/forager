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

        let item_clone = feed::clone_feed(&item);
        unsafe {
            row.set_data("data", item_clone);
        }

        list.prepend(&row);
        list.show_all();
    }
}

// Update the preview when a feed item is selected
pub fn update_preview(preview: &gtk::Box, row: &gtk::ListBoxRow) {
    let title = Label::new(Some("Title missing"));
    let description = Label::new(Some("Description missing"));
    let link = Label::new(Some("Link missing"));

    unsafe {
        let data_wrapper: Option<&feed::FeedItem> = row.get_data("data");
        if data_wrapper.is_some() {
            let data = data_wrapper.unwrap();

            title.set_label(&data.title.as_str());
            description.set_label(&data.description.as_str());
            link.set_label(&data.link.as_str());
        }
    }

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
