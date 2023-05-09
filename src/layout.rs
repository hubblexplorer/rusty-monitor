use gtk::{prelude::*, Application, ApplicationWindow, Notebook};

use crate::list_processes::{processes};

use crate::graphs::cpu_grapth;





pub fn init_layout(app: &Application){
    let window:ApplicationWindow = gtk::ApplicationWindow::new(app);
    window.set_title(Some("Layout"));
    window.set_default_size(350, 350);

    let tabs: Notebook = gtk::Notebook::new();

    let tab1 = processes();
    let label = "Processes";
    let tab_label = gtk::Label::new(Some(&label));
    tabs.append_page(&tab1, Some(&tab_label));


    let tab2 = cpu_grapth::cpu_grapth();
    let label2 = "Plots";
    let tab_label2 = gtk::Label::new(Some(&label2));
    tabs.append_page(&tab2, Some(&tab_label2));

    for i in 3..=4 {
        let label = format!("Holla {}", i);
        let tab_label = gtk::Label::new(Some(&label));
        tabs.append_page(&gtk::Box::new(gtk::Orientation::Vertical, 0), Some(&tab_label));
    }
  
    
    window.set_child(Some(&tabs));
    window.show();
    
}

