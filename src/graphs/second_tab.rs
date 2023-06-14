use gtk::{Notebook};

use crate::graphs::graphts;

pub fn create_tabs() -> Notebook
{
    let tabs: Notebook = gtk::Notebook::new();
    


    let cpu = graphts::cpu_grapth();
    let cpu_label = "Cpu";
    let cpu_label = gtk::Label::new(Some(&cpu_label));
    tabs.append_page(&cpu, Some(&cpu_label));

    let ram = graphts::ram_graph();
    let ram_label = "Ram";
    let ram_label = gtk::Label::new(Some(&ram_label));
    tabs.append_page(&ram, Some(&ram_label));




    tabs
}

