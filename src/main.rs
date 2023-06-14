use gtk::{prelude::*};
mod layout;
mod list_processes;
use layout::*;

mod graphs{
    pub mod graphts;
    pub mod second_tab;
 
}
mod list_ctl{
   
}

fn main() {
    let application =
    gtk::Application::new(Some("com.list"), Default::default());
    
    application.connect_activate(init_layout);
    application.run();

}


