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

mod info_page{
    pub mod info_page;
}


fn main() {
    gtk::init().unwrap();
    let application =
    gtk::Application::new(Some("com.rusty-monitor"), Default::default());
    
    
    application.connect_activate(init_layout);
    application.run();

}


