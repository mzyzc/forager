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

        unsafe {
            row.set_data("data", item.clone());
        }

        list.prepend(&row);
        list.show_all();
    }
}

// Update the preview when a feed item is selected
pub fn update_preview(preview: &gtk::Box, row: &gtk::ListBoxRow) {
    // Set fallback values
    let title = Label::new(Some("[Title could not be displayed]"));
    let description = Label::new(Some("[Description could not be displayed]"));
    let link = Label::new(Some("[Link could not be displayed]"));

    unsafe {
        let data_wrapper: Option<&feed::FeedItem> = row.get_data("data");
        if data_wrapper.is_some() {
            let data = data_wrapper.unwrap();

            title.set_label(&data.title);
            description.set_label(&data.description);
            link.set_label(&data.link);
        }
    }

    let preview_items = preview.get_children();
    for item in preview_items.iter() {
        preview.remove(item);
    }

    preview.add(&title);
    preview.add(&description);
    preview.add(&link);

    preview.show_all();
}
