use gtk::{prelude::*, Application, ApplicationWindow, Notebook};

use crate::{list_processes::{processes}, graphs::second_tab, list_ctl, info_page};


pub fn init_layout(app: &Application){
    let window:ApplicationWindow = gtk::ApplicationWindow::new(app);
    window.set_title(Some("Rusty monitor"));
    window.maximize();

    window.set_icon_name(Some("rusty-monitor")); 
   
    let provider = gtk::CssProvider::new();
    provider.load_from_data(include_str!("style.css"));

    // Add the provider to the default screen
    gtk::StyleContext::add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );


    let tabs: Notebook = gtk::Notebook::new();

    let tab1 = processes();
    let label = "Processes";
    let tab_label = gtk::Label::new(Some(&label));
    tabs.append_page(&tab1, Some(&tab_label));
    



    
    //let tab2 = cpu_grapth::cpu_grapth();
    let tab2 = second_tab::create_tabs();
    let label2 = "Plots";
    let tab_label2 = gtk::Label::new(Some(&label2));
    tabs.append_page(&tab2, Some(&tab_label2));
    


    let tab3 = list_ctl::list_ctl::systemctl_list();
    let label3 = "Systemctl";
    let tab_label3 = gtk::Label::new(Some(&label3));
    tabs.append_page(&tab3, Some(&tab_label3));
    

    let tab4 = info_page::info_page::info_page();
    let label4 = "System Info";
    let tab_label4 = gtk::Label::new(Some(&label4));
    tabs.append_page(&tab4, Some(&tab_label4));
    
    
  
    
    window.set_child(Some(&tabs));
    window.show();
    
}

