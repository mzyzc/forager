mod events;
mod feed;

use gtk::prelude::*;
use gtk;
use gio::prelude::*;

fn main() {
    let application = gtk::Application::new(
        Some("com.mzyzc.forager"),
        Default::default(),
    );


    application.connect_activate(|app| {
        let ui_xml = include_str!("ui.glade");
        let builder = gtk::Builder::from_string(ui_xml);
        let window: gtk::Window = builder.object("window").unwrap();
        window.set_application(Some(app));
        setup_signals(&builder);

        window.show_all();
    });

    application.run();
}

fn setup_signals(builder: &gtk::Builder) {
    let list: gtk::ListBox = builder.object("list").unwrap();
    let preview = builder.object("preview").unwrap();

    list.connect_row_selected(move |_, y| {
        if y.is_some() {
            let row = y.unwrap();
            events::update_preview(&preview, &row);
        }
    });

    let button: gtk::Button = builder.object("submit").unwrap();
    let entry: gtk::Entry = builder.object("entry").unwrap();

    button.connect_clicked(move |_| {
        events::update_list(&list, &entry.buffer().text())
    });

}
