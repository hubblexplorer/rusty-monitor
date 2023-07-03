use std::time::Duration;

use gtk::prelude::*;
use gtk::{ ScrolledWindow,DrawingArea};
use sysinfo::{CpuExt, System, SystemExt};



pub fn cpu_grapth() -> ScrolledWindow{
    let ret_window = ScrolledWindow::new();
    let drawing_area = DrawingArea::new();

    // Connect to the `draw` signal of the drawing area to draw on it

    // Create a channel for sending messages between threads

    let x_axis = (0..=29).collect::<Vec<_>>();
    let mut y_axis = vec![0.0; 30];

    let mut prev_height = 0;
    let mut prev_width = 0;
    let mut system = System::new_all();

    let mut pointer: i32 = 29;
    let mut end = false;

    drawing_area.set_draw_func(move |_, cr, height, width| {
        if height != prev_height || width != prev_width {
            // Update the previous height and width
            prev_height = height;
            prev_width = width;
        } else {
            if pointer < 0 {
                pointer = 29;
                end = true;
            }
            system.refresh_cpu();
            if !end {
                y_axis[pointer as usize] = system.global_cpu_info().cpu_usage();
                pointer -= 1;
            } else {
                // Remove the first element
                y_axis.remove(0);

                // Shift all other elements left
                for i in 0..y_axis.len()-1 {
                    y_axis[i] = y_axis[i + 1];
                }

                y_axis.push(system.global_cpu_info().cpu_usage());

               
            }
        }

       
        let padding = 30.0;
        let chart_area: (f64, f64) =
            (height as f64 - padding * 2.0, width as f64 - padding * 2.0);

        cr.set_source_rgb(0.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0); // Background color
        cr.paint().expect("Error drawing");

        // Set a monospace font

        cr.select_font_face(
            "monospace",
            gtk::cairo::FontSlant::Normal,
            gtk::cairo::FontWeight::Bold,
        );

        cr.set_font_size(12.0);

        cr.set_line_width(1.0);

        let max_x = 29;

        let max_y = 100;

        let size_x = chart_area.0 / max_x as f64;

        let size_y = chart_area.1 / max_y as f64;

        let data_points = x_axis.iter().zip(y_axis.iter());

        let normalized_data: Vec<(f64, f64, f64)> = data_points
            .map(|(x, y)| {
                (
                    padding + size_x * *x as f64,
                    padding + chart_area.1 - size_y * *y as f64,
                    *y as f64,
                )
            })
            .collect();
        cr.set_source_rgb(100.0 / 255.0, 100.0 / 255.0, 100.0 / 255.0); // Set the grid lines color

        for y_grid_line in 0..=(max_y as i32) {
            if y_grid_line % 10 == 0 {
                let y_line = y_grid_line as f64 * size_y + padding;
                cr.move_to(padding, y_line);
                cr.line_to(height as f64 - padding, y_line);
                cr.stroke().expect("Error drawing");
                cr.move_to(padding / 3.0, y_line);

                let text = format!("{}% ", (max_y - y_grid_line).to_string());
                cr.show_text(&text).expect("Error drawing");
            }
        }

        for x_grid_line in 0..=(max_x as i32) {
            if x_grid_line % 5 == 0 {
                let x_line = x_grid_line as f64 * size_x + padding;
                cr.move_to(x_line, padding);
                cr.line_to(x_line, width as f64 - padding);
                cr.stroke().expect("Error drawing");
                cr.line_to(x_line - 2.0, width as f64 - padding / 3.0);
                cr.show_text((max_x - x_grid_line + 1).to_string().as_ref())
                    .expect("Error drawing");
            }
        }

        cr.set_line_width(2.0);
        cr.set_source_rgb(0.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0); // Chart line/label color

        let data_window = normalized_data.windows(2);

        for points in data_window {
            let source = points[0];
            let target = points[1];

            if target.2 != 0.0
            {
               
                // Draw the line
                cr.move_to(source.0, source.1);
                cr.line_to(target.0, target.1);
                cr.stroke().expect("Error drawing");
            }
        }

        gtk::Inhibit(false);
    });

    ret_window.set_child(Some(&drawing_area));
    // Create a timer that calls queue_draw() on the drawing area every second
    glib::timeout_add_local(Duration::from_millis(1000), move || {
        drawing_area.queue_draw();
        glib::Continue(true)
    });
   

    ret_window
}

pub fn ram_graph() -> ScrolledWindow{
    let ret_window = ScrolledWindow::new();
    let drawing_area = DrawingArea::new();

    let x_axis = (0..=29).collect::<Vec<_>>();
    let mut y_axis = vec![0.0; 30];

    let mut prev_height = 0;
    let mut prev_width = 0;
    let mut system = System::new_all();

    let mut pointer: i32 = 29;
    let mut end = false;

    drawing_area.set_draw_func(move |_, cr, height, width| {
        if height != prev_height || width != prev_width {
            // Update the previous height and width
            prev_height = height;
            prev_width = width;
        } else {
            if pointer < 0 {
                pointer = 29;
                end = true;
            }
            system.refresh_memory();
            if !end {
                y_axis[pointer as usize] = (system.used_memory() / 10_u64.pow(6)) as f64;
                pointer -= 1;
            } else {
                // Remove the first element
                y_axis.remove(0);

                // Shift all other elements left
                for i in 0..y_axis.len()-1 {
                    y_axis[i] = y_axis[i + 1];
                }

                y_axis.push((system.used_memory() / 10_u64.pow(6)) as f64);

               
            }
        }

       
        let padding = 30.0;
        let chart_area: (f64, f64) =
            (height as f64 - padding * 2.0, width as f64 - padding * 2.0);

        cr.set_source_rgb(0.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0); // Background color
        cr.paint().expect("Error drawing");

        // Set a monospace font

        cr.select_font_face(
            "monospace",
            gtk::cairo::FontSlant::Normal,
            gtk::cairo::FontWeight::Bold,
        );

        cr.set_font_size(12.0);

        cr.set_line_width(1.0);

        let max_x = 29;

        let max_y = system.total_memory() / 10_u64.pow(6);

        let size_x = chart_area.0 / max_x as f64;

        let size_y = chart_area.1 / max_y as f64;

        let data_points = x_axis.iter().zip(y_axis.iter());

        let normalized_data: Vec<(f64, f64, f64)> = data_points
            .map(|(x, y)| {
                (
                    padding + size_x * *x as f64,
                    padding + chart_area.1 - size_y * *y as f64,
                    *y as f64,
                )
            })
            .collect();
        cr.set_source_rgb(79.0 / 255.0, 134.0 / 255.0, 140.0 / 255.0); // Set the grid lines color

        for y_grid_line in 0..=(max_y) {
            if y_grid_line % 5000 == 0 {
                let y_line = y_grid_line as f64 * size_y + padding;
                cr.move_to(padding, y_line);
                cr.line_to(height as f64 - padding, y_line);
                cr.stroke().expect("Error drawing");
                cr.move_to(padding / 3.0, y_line);

                let text = format!("{} mb", (max_y - y_grid_line).to_string());
                cr.show_text(&text).expect("Error drawing");
            }
        }

        for x_grid_line in 0..=(max_x) {
            if x_grid_line % 5 == 0 {
                let x_line = x_grid_line as f64 * size_x + padding;
                cr.move_to(x_line, padding);
                cr.line_to(x_line, width as f64 - padding);
                cr.stroke().expect("Error drawing");
                cr.line_to(x_line - 2.0, width as f64 - padding / 3.0);
                cr.show_text((max_x - x_grid_line + 1).to_string().as_ref())
                    .expect("Error drawing");
            }
        }

        cr.set_line_width(2.0);
        cr.set_source_rgb(0.0 / 255.0, 200.0 / 255.0, 255.0 / 255.0); // Chart line/label color

        let data_window = normalized_data.windows(2);

        for points in data_window {
            let source = points[0];
            let target = points[1];

            // Draw the line
            cr.move_to(source.0, source.1);
            cr.line_to(target.0, target.1);
            cr.stroke().expect("Error drawing");

        }

        gtk::Inhibit(false);
    });

    ret_window.set_child(Some(&drawing_area));
    // Create a timer that calls queue_draw() on the drawing area every second
    glib::timeout_add_seconds_local(1, move || {
        drawing_area.queue_draw();
        glib::Continue(true)
    });

    ret_window
}

