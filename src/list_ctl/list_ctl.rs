use glib::{MainContext, PRIORITY_DEFAULT, clone};
use gtk::{prelude::*, CellRendererText, ListStore, ScrolledWindow, TreeView, TreeViewColumn};


use crate::systemctl::{self, *};
use rayon::prelude::*;
use std::cell::{RefCell};
use std::process::Command;
use std::rc::Rc;
use std::thread;


//Funcion responsible for getting process information
/*fn getinfo(system: &System) -> Vec<Info> {

    let  processes: Vec<_> = system.processes().iter().collect();
    let mut ret: Vec<Info> = Vec::new();

    for (pid, process) in processes {
        let cpu_usage = process.cpu_usage() / system.cpus().len() as f32;

            let aux: Info = Info::new(*pid, process.name().to_string(), cpu_usage);

            ret.push(aux);

    }
    ret.sort_by(|a , b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
    return ret;
}*/

fn reload_data() -> Vec<(String, String)> {
    let deamons = systemctl::list_units(None, None).unwrap();
    let result: Vec<(String, String)> = deamons
        .par_iter() // Use parallel iterator
        .filter_map(|daemon| {
            let unit = daemon.as_str();

            if let Ok(unit) = Unit::from_systemctl(&unit) {
                if let Ok(_active) = unit.status() {
                    Some((unit.name, unit.active.to_string()))
                } else {
                    None
                } // Convert bool to String
            } else {
                None
            }
        })
        .collect();

    result
}
//Fuction responsible for creating the page of processes
pub fn systemctl_list() -> gtk::Grid {
    let grid = gtk::Grid::new();

    let search = gtk::SearchEntry::new();
    search.set_placeholder_text(Some("Find a deamon"));
    search.set_property("halign", gtk::Align::Center);
    search.set_editable(true);

    //Create the ListStore that will save the information of processes in the ScrolledWindow
    let list_processes = ListStore::new(&[String::static_type(),String::static_type()]);

    let searchclone = search.clone();

    let filter = gtk::TreeModelFilter::new(&list_processes,None);

    
    filter.set_visible_func(move |model , iter| {
        let searchclone = searchclone.text().to_lowercase();
        if searchclone == "" {
            true 
        }
        else {
        
            if let Ok(value) = model.get_value(iter, 0).get::<String>(){
                let value = value.as_str().to_lowercase();
           
              value.contains(&searchclone)
            } 
            else  {
                false
            }
        }
        
    });
    let filter_clone = filter.clone();
    search.connect_changed(move |_|{ filter_clone.refilter()});

    let model = gtk::TreeModelSort::with_model(&filter);

    let tree_view = TreeView::with_model(&model);
    tree_view.add_css_class("tree_view");

    //Columns
    //--------------------------------------------------------------------------------------
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    column.set_title("Name");
    column.set_sort_column_id(0);
    column.set_sort_indicator(true);
    column.set_clickable(true);
    tree_view.append_column(&column);

    let column_name = TreeViewColumn::new();
    let cell_name = CellRendererText::new();
    column_name.pack_start(&cell_name, true);
    column_name.add_attribute(&cell_name, "text", 1);
    column_name.set_title("Active");
    column_name.set_sort_indicator(true);
    column_name.set_sort_column_id(1);
    column_name.set_clickable(true);
    tree_view.append_column(&column_name);

    /*let column = TreeViewColumn::new();
    let cell = CellRendererText::new();
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 2);
    column.set_title("Cpu Usage");
    column.set_sort_indicator(true);
    column.set_sort_column_id(2);
    column.set_clickable(true);
    tree_view.append_column(&column);*/
    //--------------------------------------------------------------------------------------
    let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);

    let sender_clone = sender.clone();
    thread::spawn(move || {
        
        let info = reload_data();

        sender_clone.send(info).expect("Error sending message");
            
        
    });
    

    // The main loop executes the closure as soon as it receives the message
    let tree_view_clone = tree_view.clone();
    receiver.attach(
        None,
        clone!(@weak  list_processes => @default-return Continue(false),
                    move |info| {       
                      // Get the TreeSelection object from the tree_view_clone
                    let selection = tree_view_clone.selection();

                    // Get the selected row
                    let selected_row = selection.selected().map(|(model, iter)| {
                        model.get_value(&iter, 0).get::<String>().unwrap()
                    }).unwrap_or(String::from("-1"));
                  
                        list_processes.clear();
                        let mut count = 0;

                        for i in info {
                            
                            
                            list_processes.insert_with_values(Some(count), &[(0, &i.0.to_string()), (1,&i.1.to_string())] );
                            if i.0.to_string() == selected_row{
                                // Set the cursor (and selection) to the specified row
                                tree_view_clone.set_cursor_from_name(Some(&selected_row));
                            }
                            count +=1;
                        }
                       
                        Continue(true)
                    }
        ),
    );
   

    //Menu for processes
    //------------------------------------------------------------------------------------------

    let row_data_ref = Rc::new(RefCell::new(Vec::new()));

    //Start button
    let start_button = gtk::Button::new();
    let popover_menu = gtk::Popover::new();
    let list_menu = gtk::ListBox::new();

    start_button.set_label("Start");
    popover_menu.set_child(Some(&list_menu));

    list_menu.append(&start_button);

    let row_data_ref_clone = Rc::clone(&row_data_ref);
    let popover_menu_clone = popover_menu.clone();
   
    let sender_clone = sender.clone();
    let list_processes_clone = list_processes.clone();
    //Actions for the start button
    start_button.connect_clicked(move |_| {
        let sender_clone = sender_clone.clone();
        let row_data: std::cell::Ref<Vec<String>> = row_data_ref_clone.borrow();
        let pid = row_data.first().unwrap();
        let pid = &pid[..];

        let mut output = Command::new("pkexec")
            .args(["systemctl", "start", pid])
            .spawn()
            .expect("failed to execute process");
      
        if output.stderr.is_some() {
            println!("{:?}", output.stderr);
        } else {
            //edit row data
        
            list_processes_clone.clear();
            thread::spawn(move || {
                
                output.wait().unwrap();
                let info = reload_data();
        
                sender_clone.send(info).expect("Error sending message");
                    
                
            });

        }

        popover_menu_clone.hide()
    });

    let stop_button = gtk::Button::new();


    stop_button.set_label("Stop");
    popover_menu.set_child(Some(&list_menu));

    list_menu.append(&stop_button);

    let row_data_ref_clone = Rc::clone(&row_data_ref);
    let popover_menu_clone = popover_menu.clone();
   
    let sender_clone = sender.clone();
    //Actions for the start button
    stop_button.connect_clicked(move |_| {
        let sender_clone = sender_clone.clone();
        let row_data: std::cell::Ref<Vec<String>> = row_data_ref_clone.borrow();
        let pid = row_data.first().unwrap();
        let pid = &pid[..];

        let mut output = Command::new("pkexec")
            .args(["systemctl", "stop", pid])
            .spawn()
            .expect("failed to execute process");
      
        if output.stderr.is_some() {
            println!("{:?}", output.stderr);
        } else {
            //edit row data
        
            list_processes.clear();
            thread::spawn(move || {
                
                output.wait().unwrap();
                let info = reload_data();
        
                sender_clone.send(info).expect("Error sending message");
                    
                
            });

        }

        popover_menu_clone.hide()
    });

    popover_menu.set_parent(&tree_view);

    //Set left click as input
        let gesture_click = gtk::GestureClick::new();
        gesture_click.set_propagation_phase(gtk::PropagationPhase::Capture);
        gesture_click.set_button(gtk::gdk::ffi::GDK_BUTTON_SECONDARY as u32);
        tree_view.add_controller(gesture_click.clone());
        let tree_view_clone = tree_view.clone();

        //Connecter to the right button
        gesture_click.connect_pressed(move |_gesture_click, _n_press, x, y| {
            if let Some((path, _)) = tree_view_clone.dest_row_at_pos(x as i32, y as i32) {
                let path = path.unwrap();
                let model = tree_view_clone.model().unwrap();
                let iter = model.iter(&path).unwrap();

                // Get the data of the process in the row from model
                let column_count = model.n_columns();
                println!("Column count: {}", column_count);
                let mut row_data = Vec::new();
                for i in 0..column_count {
                    let value = model.get_value(&iter, i);
                    row_data.push(value.get::<String>().unwrap());
                }

                // Print the data in the row
                // println!("Clicked on row: {:?}, data: {:?}", path.to_str(), row_data);

                //Send data to button
                *row_data_ref.borrow_mut() = row_data;

                //Open popup
                popover_menu
                    .set_pointing_to(Some(&gtk::gdk::Rectangle::new(x as i32, y as i32, 1, 1)));

                popover_menu.popup();
            } 
        });

    //--------------------------------------------------------------------------------------------

    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Always);
    scrolled_window.set_child(Some(&tree_view));
    scrolled_window.set_hexpand(true);
    scrolled_window.set_vexpand(true);

    grid.attach(&search, 0, 0, 1, 1);
    grid.attach(&scrolled_window, 0, 1, 1, 10);
    grid
}
