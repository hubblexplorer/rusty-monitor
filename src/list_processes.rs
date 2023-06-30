use gtk::{prelude::*, ScrolledWindow, ListStore, TreeView, TreeViewColumn, CellRendererText, SortType, SortColumn, TreeModelSort, TreeModelFilter, SearchEntry, Grid, Align, Button, Popover, ListBox, GestureClick, PropagationPhase, gdk, PolicyType};

use gtk::glib::{clone, MainContext, PRIORITY_DEFAULT};
use std::cell::RefCell;

use std::process::Command;
use std::rc::Rc;
use std::{thread, time::Duration};
use sysinfo::{Pid, ProcessExt, System, SystemExt};


fn format_memory_usage(memory_usage: u64) -> String {
    const BASE: u64 = 1024;
    
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = memory_usage as f64;
    let mut unit_index = 0;
    while size >= BASE as f64 && unit_index < units.len() - 1 {
        size /= BASE as f64;
        unit_index += 1;
    }
    format!("{:.2} {}", size, units[unit_index])
}


//Helper Struct
#[derive(Clone, Debug)]
struct Info {
    pid: Pid,
    name: String,
    cpu_usage: f32,
    memory: u64,
    disk_usage: u64,
    status: String
}

//Inicializer of struct
impl Info {
    //Callable new for struck
    fn new(pid: Pid, name: String, cpu_usage: f32, memory: u64, disk_usage: u64, status: String) -> Info {
        Info {
            pid,
            name,
            cpu_usage,
            memory,
            disk_usage,
            status

        }
    }
}

//Funcion responsible for getting process information
fn getinfo(system: &System) -> Vec<Info> {
  
    let  processes: Vec<_> = system.processes().iter().collect();
    let mut ret: Vec<Info> = Vec::new();

    for (pid, process) in processes {
        let cpu_usage = process.cpu_usage() / system.cpus().len() as f32;
            let aux: Info = Info::new(*pid, process.name().to_string(), cpu_usage, process.memory(), process.disk_usage().total_written_bytes, process.status().to_string());
            
            ret.push(aux);
        
    }
    return ret;
}

//Fuction responsible for creating the page of processes
pub fn processes ()-> Grid{


    let grid  = Grid::new();

    let search = SearchEntry::new();
    search.set_placeholder_text(Some("Find a process"));
    search.set_property("halign", Align::Center);
    search.set_editable(true);

   
    //Create the ListStore that will save the information of processes in the ScrolledWindow
    let list_processes = ListStore::new(&[u64::static_type(),String::static_type(),f32::static_type(),u64::static_type(),u64::static_type(),String::static_type()]);

    let searchclone = search.clone();
    
     let filter = TreeModelFilter::new(&list_processes,None);

   
    filter.set_visible_func(move |model , iter| {
        let searchclone = searchclone.text().to_lowercase();
        if searchclone == "" {
            true 
        }
        else {
        
            if let Ok(value) = model.get_value(iter,1).get::<String>(){
                let value = value.as_str().to_lowercase();
           
              value.contains(&searchclone)
            } 
            else  {
                false
            }
        }
    
        
    });



    

    let model = TreeModelSort::with_model(&filter);
    model.set_sort_column_id(SortColumn::Index(2), SortType::Descending);
    /*let model = list_processes.upcast_ref::<TreeSortable>();

    model.set_sort_func(gtk::SortColumn::Index(3),|model, iter1, iter2| {
        let value1: String = model.get_value(iter1, 3).get().unwrap();
        let value2: String = model.get_value(iter2, 3).get().unwrap();
    
    
        println!("{} {}", value1, value2);
    
        // Extract memory usage numbers from the formatted strings
        let usage1 = value1.split_whitespace().next().and_then(|s| s.parse::<f64>().ok());
        let usage2 = value2.split_whitespace().next().and_then(|s| s.parse::<f64>().ok());
    
      
        if let (Some(usage1), Some(usage2)) = (usage1, usage2) {
            println!("{} {}", usage1, usage2);
            // Compare memory usage numbers
            usage1.total_cmp(&usage2).into()
        } else {
            // Fallback to comparing the original strings if memory usage parsing fails
            value1.cmp(&value2).into()
        }
    });*/

    

   /*  let filter = gtk::TreeModelFilter::new(model,None);

   
    filter.set_visible_func(move |model , iter| {
        let searchclone = searchclone.text().to_lowercase();
        if searchclone == "" {
            true 
        }
        else {
        
            if let Ok(value) = model.get_value(iter,1).get::<String>(){
                let value = value.as_str().to_lowercase();
           
              value.contains(&searchclone)
            } 
            else  {
                false
            }
        }
    
        
    });*/

   
    
   
  
    let tree_view = TreeView::with_model(&model);
    

   
    
    //tree_view.add_css_class("tree_view");

   

    //Columns
    //--------------------------------------------------------------------------------------
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    column.set_title("PID");
    column.set_sort_column_id(0);
    column.set_sort_indicator(true);
    column.set_clickable(true);
    column.set_resizable(true);
    tree_view.append_column(&column);


    
    let column_name = TreeViewColumn::new();
    let cell_name = CellRendererText::new();
    column_name.pack_start(&cell_name, true);
    column_name.add_attribute(&cell_name, "text", 1);
    column_name.set_title("Name");
    column_name.set_sort_indicator(true);
    column_name.set_sort_column_id(1);
    column_name.set_clickable(true);
    column.set_resizable(true);
    tree_view.append_column(&column_name);

    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();
    
    
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 2);
    column.set_title("Cpu Usage");
    column.set_sort_indicator(true);
    column.set_sort_column_id(2);
    column.set_clickable(true);
    column.set_resizable(true);
    column.set_sort_order(SortType::Descending);

    column.set_cell_data_func(&cell, |_, cell, model, iter| {
        let value = model.get_value(iter, 2).get::<f32>().unwrap() ;
        let text = format!("{:.2}%", value);
        cell.set_property("text",Some(&text));
        
    });

    
    tree_view.append_column(&column);


    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 3);
    column.set_title("Memory");
    column.set_sort_indicator(true);
    column.set_sort_column_id(3);
    column.set_clickable(true);
    column.set_resizable(true);

    column.set_cell_data_func(&cell, |_, cell, model, iter| {
        let value = model.get_value(iter, 3).get::<u64>().unwrap() ;
        let formatted = format_memory_usage(value);
        cell.set_property("text",Some(&formatted));
            
        
        
    });
    tree_view.append_column(&column);


    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 4);
    column.set_title("Disk Usage");
    column.set_sort_indicator(true);
    column.set_sort_column_id(4);
    column.set_clickable(true);
    column.set_resizable(true);
    column.set_cell_data_func(&cell, |_, cell, model, iter| {
        let value = model.get_value(iter, 4).get::<u64>().unwrap() ;
        let formatted = format_memory_usage(value);
        cell.set_property("text",Some(&formatted));
            
        
        
    });
    tree_view.append_column(&column);


    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();
    cell.set_property("text", "value%");
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 5);
    column.set_title("Status");
    column.set_sort_indicator(true);
    column.set_sort_column_id(5);
    column.set_clickable(true);
    column.set_resizable(true);
    tree_view.append_column(&column);
     //--------------------------------------------------------------------------------------

    let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);

    let sender_clone = sender.clone();
    // The long running operation runs now in a separate thread

    thread::spawn( move || {
        let mut system = System::new_all();
     
        loop {

           
            system.refresh_processes();

            let info = getinfo(&system);

            sender_clone.send(info).expect("Error sending message");
            thread::sleep(Duration::new(2, 500));
           
        }
    });

    // The main loop executes the closure as soon as it receives the message
    
    receiver.attach(
        None,
        clone!(@weak  list_processes => @default-return Continue(false),
                    move |info| {       
                      // Get the TreeSelection object from the tree_view_clone

                       
                        

                        list_processes.clear();
                        let mut count = 0;

                        for i in info {
                           // let cpu_usage = format!("{:.4}%", (i.cpu_usage).to_string());
                            //let memory = format_memory_usage(i.memory);
                            
                            let pid: i32 = i.pid.to_string().parse().unwrap();
                            list_processes.insert_with_values(Some(count), &[(0, &pid), (1,&i.name.to_string()), (2,&i.cpu_usage),(3,&i.memory),(4,&i.disk_usage),(5,&i.status.to_string())] );
                           
                            count +=1;
                        }
                       
                       
                        Continue(true)
                    }
        ),
    );
    //Menu for processes
    //------------------------------------------------------------------------------------------

        let row_data_ref = Rc::new(RefCell::new(Vec::new()));

        //Kill button
        let kill_button = Button::new();
        let popover_menu = Popover::new();
        let list_menu = ListBox::new();

        kill_button.set_label("Kill");
        popover_menu.set_child(Some(&list_menu));

        list_menu.append(&kill_button);
       

        
        let row_data_ref_clone = Rc::clone(&row_data_ref);
        let popover_menu_clone = popover_menu.clone();
        //Actions for the kill button
        kill_button.connect_clicked(move |_| {
    
            let row_data: std::cell::Ref<Vec<String>> = row_data_ref_clone.borrow();
            let pid = row_data.first().unwrap();
            let pid = &pid[..];

         
            let output = Command::new("kill").args(["-9",pid]).spawn()
            .expect("failed to execute process");
            if output.stderr.is_some() {
                println!("{:?}",  output.stderr);
            }
            popover_menu_clone.hide()
            
        });


        let stop_button = Button::new();

        stop_button.set_label("Stop");

        list_menu.append(&stop_button);


        let row_data_ref_clone = Rc::clone(&row_data_ref);
        let popover_menu_clone = popover_menu.clone();
        //Actions for the kill button
        stop_button.connect_clicked(move |_| {
    
            let row_data: std::cell::Ref<Vec<String>> = row_data_ref_clone.borrow();
            let pid = row_data.first().unwrap();
            let pid = &pid[..];

         
            let output = Command::new("kill").args(["-19",pid]).spawn()
            .expect("failed to execute process");
            if output.stderr.is_some() {
                println!("{:?}",  output.stderr);
            }
            popover_menu_clone.hide()
            
        });



        let cont_button = Button::new();

        cont_button.set_label("Continue");

        list_menu.append(&cont_button);


        let row_data_ref_clone = Rc::clone(&row_data_ref);
        let popover_menu_clone = popover_menu.clone();
        //Actions for the kill button
        cont_button.connect_clicked(move |_| {
    
            let row_data: std::cell::Ref<Vec<String>> = row_data_ref_clone.borrow();
            let pid = row_data.first().unwrap();
            let pid = &pid[..];

         
            let output = Command::new("kill").args(["-18",pid]).spawn()
            .expect("failed to execute process");
            if output.stderr.is_some() {
                println!("{:?}",  output.stderr);
            }
            popover_menu_clone.hide()
            
        });

        popover_menu.set_parent(&tree_view);


    
        //Set left click as input
        let gesture_click = GestureClick::new();
        gesture_click.set_propagation_phase(PropagationPhase::Capture);
        gesture_click.set_button(gdk::ffi::GDK_BUTTON_SECONDARY as u32);
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
                let mut row_data = Vec::new();
                
                for i in 0..column_count {
                    let value = model.get_value(&iter, i);
                    if let Ok(value) = value.get::<String>() {
                        row_data.push(value);
                    }
                    else if let Ok(value) = value.get::<u64>() {
                        row_data.push(value.to_string());
                    }
                    else if let Ok(value) = value.get::<f32>() {
                        row_data.push(value.to_string());
                    }
                    else {
                        ()
                    }
                }

                // Print the data in the row
                // println!("Clicked on row: {:?}, data: {:?}", path.to_str(), row_data);

                //Send data to button
                *row_data_ref.borrow_mut() = row_data;

                //Open popup
                popover_menu
                    .set_pointing_to(Some(&gdk::Rectangle::new(x as i32, y as i32, 1, 1)));

                popover_menu.popup();
            } 
        });


        


      
    

       
        
       
        
    //--------------------------------------------------------------------------------------------
 
    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_policy(PolicyType::Automatic, PolicyType::Always);
    scrolled_window.set_child(Some(&tree_view));
    scrolled_window.set_hexpand(true);
    scrolled_window.set_vexpand(true);
    
   
 
    grid.attach(&search, 0,  0, 1, 1);
    grid.attach(&scrolled_window, 0, 1, 1, 10);
    grid
}
