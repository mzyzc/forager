use crate::feed;
use gtk::prelude::*;
use gtk::{ListBoxRow, Label};

pub fn update_list(list: &gtk::ListBox, url: &str) {
    let items = match feed::get_feed(url) {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };

    for item in items.iter() {
        let row = ListBoxRow::new();

        let label = Label::new(Some(&item.title));
        label.set_xalign(0.0);
        row.add(&label);

        // Unsafe block needed to set data for a widget
        unsafe {
            row.set_data("data", item.clone());
        }

        list.prepend(&row);
        list.show_all();
    }
}

pub fn update_preview(preview: &gtk::Box, row: &gtk::ListBoxRow) {
    let title = Label::new(Some("[Title could not be displayed]"));
    let description = Label::new(Some("[Description could not be displayed]"));
    let link = Label::new(Some("[Link could not be displayed]"));

    title.set_xalign(0.0);
    description.set_xalign(0.0);
    link.set_xalign(0.0);

    title.set_selectable(true);
    description.set_selectable(true);
    link.set_selectable(true);

    // Unsafe block needed to get data from a widget
    unsafe {
        //let data_wrapper: Option<&feed::FeedItem> = row.data("data");
        let data_wrapper = row.data("data");
        if data_wrapper.is_some() {
            let data: &feed::FeedItem = data_wrapper.unwrap().as_ref();

            title.set_markup(&data.title);
            description.set_markup(&data.description);
            link.set_markup(&data.link);
        }
    }

    let preview_items = preview.children();
    for item in preview_items.iter() {
        preview.remove(item);
    }

    preview.add(&title);
    preview.add(&description);
    preview.add(&link);

    preview.show_all();
}
