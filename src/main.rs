mod events;
mod feed;

use gtk::prelude::*;
use gtk;
use gio::prelude::*;

fn main() {
    let application = gtk::Application::new(
        Some("com.mzyzc.forager"),
        Default::default(),
    ).expect("failed to initialize GTK");


    application.connect_activate(|app| {
        let ui_xml = include_str!("ui.glade");
        let builder = gtk::Builder::from_string(ui_xml);
        let window: gtk::Window = builder.get_object("window").unwrap();
        window.set_application(Some(app));

        window.show_all();
    });

    application.run(&[]);
}
