use gtk::{prelude::*};
mod layout;
mod list_processes;
use layout::*;

mod graphs{
    pub mod cpu_grapth;
}

fn main() {
    let application =
    gtk::Application::new(Some("com.list"), Default::default());

    application.connect_activate(init_layout);
    application.run();
}


