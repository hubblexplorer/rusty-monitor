use gtk::{prelude::*, ScrolledWindow, ListStore, TreeView, TreeViewColumn, CellRendererText};

use gtk::glib::{clone, MainContext, PRIORITY_DEFAULT};
use std::{thread, time::Duration};
use sysinfo::{Pid, ProcessExt, System, SystemExt};

#[derive(Clone, Debug)]
struct Info {
    pid: Pid,
    name: String,
    cpu_usage: f32,
}

impl Info {
    // An associated function that creates a new instance of the struct
    fn new(pid: Pid, name: String, cpu_usage: f32) -> Info {
        Info {
            pid,
            name,
            cpu_usage,
        }
    }
}


fn getinfo(system: &System) -> Vec<Info> {
    // Refresh processes information:

    // Print processes information ordered by CPU usage:
    let  processes: Vec<_> = system.processes().iter().collect();
    
    let mut ret: Vec<Info> = Vec::new();

    for (pid, process) in processes {
        let cpu_usage = process.cpu_usage() / system.cpus().len() as f32;
  
            let aux: Info = Info::new(*pid, process.name().to_string(), cpu_usage);

            ret.push(aux);
        
    }
    ret.sort_by(|a , b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
    return ret;
}

pub fn processes ()-> ScrolledWindow{
    let list_processes = ListStore::new(&[String::static_type(),String::static_type(),String::static_type()]);

    let tree_view = TreeView::with_model(&list_processes);

    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    column.set_title("PID");


    tree_view.append_column(&column);

    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 1);
    column.set_title("Name");
    tree_view.append_column(&column);

    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 2);
    column.set_title("Cpu Usage");
    tree_view.append_column(&column);




    let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);

    let sender_clone = sender.clone();
    // The long running operation runs now in a separate thread

    thread::spawn(move || {
        let mut system = System::new_all();
        loop {
            system.refresh_processes();

            let info = getinfo(&system);

           // info.sort_by(|a,b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());


            sender_clone.send(info).expect("Error sending message");
            thread::sleep(Duration::new(2, 500));
        }
    });

    // The main loop executes the closure as soon as it receives the message
    receiver.attach(
        None,
        clone!(@weak  list_processes => @default-return Continue(false),
                    move |info| {       
                        
                        list_processes.clear();
                        let mut count = 0;

                        for i in info {
                            let cpu_usage = format!("{:.4}%", (i.cpu_usage).to_string());
                            list_processes.insert_with_values(Some(count), &[(0, &i.pid.to_string()), (1,&i.name.to_string()), (2,&cpu_usage)] );
                            count +=1;
                        }
                       
                        Continue(true)
                    }
        ),
    );

    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Always);
    scrolled_window.set_child(Some(&tree_view));
    return scrolled_window;
}