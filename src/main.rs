use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder};

fn main() {

    let builder = Builder::from_file("ui.glade");
    let app = Application::builder(builder);

    app.connect_activate(|app| {
        app.
    });

    app.run();
}
