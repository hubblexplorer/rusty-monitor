use gtk::{prelude::*, ScrolledWindow, ListStore, TreeView, TreeViewColumn, CellRendererText, TreeSortable};

use gtk::glib::{clone, MainContext, PRIORITY_DEFAULT};
use std::cell::RefCell;
use std::rc::Rc;
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

    let sortable = list_processes.upcast_ref::<TreeSortable>();

    let cell = CellRendererText::new();
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    column.set_title("PID");


    sortable.set_sort_func(gtk::SortColumn::Index(0), |sortable, iter1, iter2| {
        let val1_str: String = sortable.get_value(iter1, 0).get().unwrap();
        let val2_str: String = sortable.get_value(iter2, 0).get().unwrap();
    
        let val1 = val1_str.parse::<i32>().unwrap_or(0);
        let val2 = val2_str.parse::<i32>().unwrap_or(0);
    
        val1.cmp(&val2).into()
    
    });

    column.set_sort_column_id(0);
    column.set_sort_indicator(true);
    column.set_clickable(true);

    
    tree_view.append_column(&column);
    

   

    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 1);
    column.set_title("Name");
    column.set_sort_indicator(true);

   
    sortable.set_sort_func(gtk::SortColumn::Index(1), |sortable, iter1, iter2| {
        let val1: String = sortable.get_value(iter1, 1).get().unwrap();
        let val2: String = sortable.get_value(iter2, 1).get().unwrap();
        val1.cmp(&val2).into()
    });

    column.set_sort_column_id(1);
    column.set_sort_indicator(true);
    column.set_clickable(true);


    

    tree_view.append_column(&column);

    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 2);
    column.set_title("Cpu Usage");
    column.set_sort_indicator(true);


    sortable.set_sort_func(gtk::SortColumn::Index(2), |sortable, iter1, iter2| {
        let val1: String = sortable.get_value(iter1, 2).get().unwrap();
        let val2: String = sortable.get_value(iter2, 2).get().unwrap();
        val1.cmp(&val2).into()
    });

    column.set_sort_column_id(2);
    column.set_sort_indicator(true);
    column.set_clickable(true);


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

     //-------------------------------------------------
        let row_data_ref = Rc::new(RefCell::new(Vec::new()));

        let menu_button = gtk::Button::new();
        let popover_menu = gtk::Popover::new();

        menu_button.set_label("Edit");

        popover_menu.set_child(Some(&menu_button));

        popover_menu.set_parent(&tree_view);

        let row_data_ref_clone = Rc::clone(&row_data_ref);
        menu_button.connect_clicked(move |_| {
            // TODO: Implement the edit action here
            let row_data = row_data_ref_clone.borrow();
            println!("Edit button clicked, row data: {:?}", *row_data);
        });

        let gesture_click = gtk::GestureClick::new();
        gesture_click.set_propagation_phase(gtk::PropagationPhase::Capture);
        gesture_click.set_button(gtk::gdk::ffi::GDK_BUTTON_SECONDARY as u32);
        tree_view.add_controller(gesture_click.clone());
        let tree_view_clone = tree_view.clone();
        gesture_click.connect_pressed(move |_gesture_click, _n_press, x, y| {
            println!("Right button pressed at ({}, {})", x, y);
            if let Some((path, _)) = tree_view_clone.dest_row_at_pos(x as i32, y as i32) {
                let path = path.unwrap();
                let model = tree_view_clone.model().unwrap();
                let iter = model.iter(&path).unwrap();

                // Get the data in the row using the TreeIter
                let column_count = model.n_columns();
                let mut row_data = Vec::new();
                for i in 0..column_count {
                    let value = model.get_value(&iter, i);
                    row_data.push(value.get::<String>().unwrap());
                }

                // Print the data in the row
                println!("Clicked on row: {:?}, data: {:?}", path.to_str(), row_data);

                *row_data_ref.borrow_mut() = row_data;

                popover_menu
                    .set_pointing_to(Some(&gtk::gdk::Rectangle::new(x as i32, y as i32, 1, 1)));

                popover_menu.popup();
            } else {
                println!("No row clicked");
            }
        });
        //---------------------------------------------

    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Always);
    scrolled_window.set_child(Some(&tree_view));
    return scrolled_window;
}
