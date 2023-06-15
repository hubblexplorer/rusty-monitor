use gtk::{prelude::*};
mod layout;
mod list_processes;
use layout::*;
#[macro_use]
extern crate default_env;
mod systemctl;
mod list_ctl {
    pub mod list_ctl;
}

mod graphs{
    pub mod graphts;
    pub mod second_tab;
 
}


fn main() {
    let application =
    gtk::Application::new(Some("com.list"), Default::default());
    
    application.connect_activate(init_layout);
    application.run();

}


