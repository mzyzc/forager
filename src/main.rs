mod events;
mod feed;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Entry, Grid, MenuBar, MenuItem, Paned, Orientation, ScrolledWindow, ListBox, ButtonBox, Button};
use gio::prelude::*;

fn main() {
    let application = Application::new(
        Some("com.mzyzc.forager"),
        Default::default(),
    ).expect("failed to initialize GTK");

    application.connect_activate(|application| {
        init_ui(&application);
    });

    application.run(&[]);
}

fn init_ui(application: &gtk::Application) {
        let window = ApplicationWindow::new(application);
        window.set_title("Forager");
        window.set_default_size(800, 600);

        let grid = Grid::new();
        window.add(&grid);

        let menu_bar = MenuBar::new();
        menu_bar.add(&MenuItem::with_label("File"));
        menu_bar.add(&MenuItem::with_label("View"));
        grid.attach(&menu_bar, 0, 0, 1, 1);

        let paned = Paned::new(Orientation::Horizontal);
        paned.set_hexpand(true);
        paned.set_vexpand(true);
        paned.set_position(300);
        grid.attach(&paned, 0, 1, 1, 1);

        let scroll_win = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        paned.add1(&scroll_win);

        let preview_box = Box::new(Orientation::Vertical, 6);
        paned.add2(&preview_box);

        let list_box = ListBox::new();
        list_box.connect_row_selected(move |_, y| {
            if y.is_some() {
                let row = y.unwrap();
                events::update_preview(&preview_box, &row);
            }
        });
        scroll_win.add(&list_box);

        let link_entry = Entry::new();
        grid.attach(&link_entry, 0, 3, 1, 1);

        let button_box = ButtonBox::new(Orientation::Horizontal);
        grid.attach(&button_box, 1, 3, 1, 1);

        let button = Button::with_label("Update");
        button.connect_clicked(move |_| {
            events::update_list(&list_box, &link_entry.get_buffer().get_text());
        });
        button_box.add(&button);

        window.show_all();
}
