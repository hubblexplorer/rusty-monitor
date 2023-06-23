use std::{fs, io::ErrorKind, process::Command};

use gtk::{gdk_pixbuf::PixbufLoader, prelude::*, Grid, Image, Label, ScrolledWindow, PolicyType};

use include_dir::{include_dir, Dir};
use reqwest::blocking::get;
use scraper::{Html, Selector};
use sysinfo::{CpuExt, System, SystemExt};

fn add_empty_cells(grid: &Grid) {
    let l = Label::new(None);
    l.add_css_class("debug");
    grid.attach(&l, 0, 0, 2, 20);

    let l = Label::new(None);
    l.add_css_class("debug");
    grid.attach(&l, 5, 0, 2, 1);

    let l = Label::new(None);
    l.add_css_class("debug");
    grid.attach(&l, 5, 3, 2, 1);

    let l = Label::new(None);
    l.add_css_class("debug");
    grid.attach(&l, 3, 4, 4, 1);

    let l = Label::new(None);
    l.add_css_class("debug");
    grid.attach(&l, 7, 0, 2, 20);

    let l = Label::new(None);
    l.add_css_class("debug");
    grid.attach(&l, 5, 11, 2, 1);
}

fn get_website(dist_name: &str) -> Result<String, ErrorKind> {
    let url = format!(
        "https://distrowatch.com/table.php?distribution={}",
        dist_name
    );
    let response = get(&url).unwrap();

    if response.status().is_success() {
        let body = response.text().unwrap();

        let document = Html::parse_document(&body);
        let selector = Selector::parse("tr.Background").unwrap();

        for element in document.select(&selector) {
            let text = element.text().collect::<Vec<_>>();
            if text.contains(&"Home Page") {
                let link_element = text.get(3).unwrap();

                return Ok(link_element.to_string());
            }
        }
        return Err(ErrorKind::Other);
    } else {
        return Err(ErrorKind::Other);
    }
}

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

pub fn info_page() -> ScrolledWindow {
    const ASSETS_DIR: Dir<'_> = include_dir!("src/assets/");

    let os_release_contents = fs::read_to_string("/etc/os-release").unwrap();

    let system = System::new_all();

    let id = os_release_contents
        .lines()
        .find(|line| line.starts_with("ID="))
        .unwrap()
        .split('=')
        .nth(1)
        .unwrap()
        .trim_matches('"');

    let text = match id {
        "ubuntu" => "Ubuntu",
        "fedora" => "Fedora",
        "arch" => "Arch Linux",
        "debian" => "Debian",
        "opensuse" => "OpenSUSE",
        "rhel" => "Red Hat Enterprise Linux",
        "centos" => "CentOS",
        "gentoo" => "Gentoo",
        "void" => "Void Linux",
        "alpine" => "Alpine Linux",
        "clear-linux-os" => "Clear Linux",
        "solus" => "Solus",
        "nixos" => "NixOS",
        "artix" => "Artix Linux",
        "manjaro" => "Manjaro",
        "antergos" => "Antergos",
        "endeavouros" => "EndeavourOS",
        "mx" => "MX Linux",
        "kali" => "Kali Linux",
        "parrot" => "Parrot OS",
        "tails" => "Tails",
        "qubes" => "Qubes OS",
        "puppy" => "Puppy Linux",
        "slackware" => "Slackware",
        "raspbian" => "Raspbian",
        "ubuntu-mate" => "Ubuntu MATE",
        "ubuntu-unity" => "Ubuntu Unity",
        "ubuntu-cinnamon" => "Ubuntu Cinnamon",
        "lubuntu" => "Lubuntu",
        "xubuntu" => "Xubuntu",
        "kubuntu" => "Kubuntu",
        "ubuntu-studio" => "Ubuntu Studio",
        "ubuntu-budgie" => "Ubuntu Budgie",
        "ubuntu-kylin" => "Ubuntu Kylin",
        "ubuntu-mate-next" => "Ubuntu MATE Next",
        "ubuntu-dde" => "UbuntuDDE",
        "neon" => "KDE neon",
        "feren" => "Feren OS",
        "bluestar" => "BlueStar",
        "garuda" => "Garuda Linux",
        "mabox" => "MaBox",
        "openmandriva" => "OpenMandriva",
        "pclinuxos" => "PCLinuxOS",
        "peppermint" => "Peppermint OS",
        "pop" => "Pop!_OS",
        "redstar" => "RedStar OS",
        "rosa" => "ROSA Linux",
        "reborn" => "RebornOS",
        "tinycore" => "Tiny Core Linux",
        "endless" => "Endless OS",
        "deepin" => "Deepin",
        "kaos" => "Kaos",
        "knoppix" => "Knoppix",
        _ => "Unknown distribution",
    };

    let grid = Grid::new();
    grid.set_vexpand(false);
    grid.set_hexpand(false);
    grid.set_row_homogeneous(true);
    grid.set_column_homogeneous(true);

    grid.add_css_class("debug");

    // Distro --------------------------------------------------------
    let filename = format!("512/512_{}.svg", id);

    let data = ASSETS_DIR.get_file(filename).unwrap().contents();

    let loader = PixbufLoader::new();
    loader.write(&data).expect("Failed to write file");
    loader.close().expect("Failed to close file");

    let pixbuf = loader.pixbuf().unwrap();
    let image = Image::from_pixbuf(Some(&pixbuf));
    image.add_css_class("debug");
    grid.attach(&image, 3, 0, 2, 4);

    let l = Label::new(Some(text));
    l.add_css_class("debug");
    grid.attach(&l, 5, 1, 2, 1);

    if let Ok(link) = get_website(id) {
        let l = Label::new(Some(link.as_str()));
        l.add_css_class("debug");
        grid.attach(&l, 5, 2, 2, 1);
    } else {
        let l = Label::new(Some("No url found"));
        l.add_css_class("debug");
        grid.attach(&l, 5, 2, 2, 1);
    }

    let l = Label::new(Some("Hardware"));
    l.add_css_class("bold");
    grid.attach(&l, 3, 5, 4, 1);

    // CPU-----------------------------------------------------------
    let l = Label::new(Some("Processor: "));
    l.add_css_class("debug");
    grid.attach(&l, 3, 6, 2, 1);

    let cpu_name = system.global_cpu_info().brand();

    let l = Label::new(Some(cpu_name));
    l.add_css_class("debug");
    grid.attach(&l, 5, 6, 2, 1);

    // Memory-----------------------------------------------------------
    let l = Label::new(Some("Memory: "));
    l.add_css_class("debug");
    grid.attach(&l, 3, 7, 2, 1);

    let ram = system.total_memory();

    let ram = format_memory_usage(ram);

    let l = Label::new(Some(ram.as_str()));
    l.add_css_class("debug");
    grid.attach(&l, 5, 7, 2, 1);

    // Graphics-----------------------------------------------------------
    let graphics = {
        let output = Command::new("lspci")
            .arg("-v")
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();

        let filtered_lines: Vec<&str> = lines
            .iter()
            .filter(|line| {
                line.to_lowercase().contains("vga")
                    || line.to_lowercase().contains("3d")
                    || line.to_lowercase().contains("2d")
            })
            .map(|line| *line)
            .collect();

        let output_str = filtered_lines.join("\n");

        // Process each line and format the graphics card information
        let graphics_lines: Vec<String> = output_str
            .lines()
            .map(|line| {
                let line = line.trim();

                if let Some(index) = line.find(": ") {
                    let line = &line[index + 1..];
                    if let Some(index) = line.find("(") {
                        line[..index].to_string()
                    } else {
                        line.to_string()
                    }
                } else {
                    line.to_string()
                }
            })
            .collect();

        let graphics_str = graphics_lines.join("\n");
        graphics_str
    };

    let l = Label::new(Some("Graphics"));
    l.add_css_class("debug");
    grid.attach(&l, 3, 8, 2, 1);

    let l = Label::new(Some(&graphics));
    l.add_css_class("debug");
    grid.attach(&l, 5, 8, 2, 1);

    // Manufacturer-----------------------------------------------------------
    let manufacturer = Command::new("cat")
        .arg("/sys/class/dmi/id/product_name")
        .output()
        .expect("Not found");

    let l = Label::new(Some("Manufacturer: "));
    l.add_css_class("debug");
    grid.attach(&l, 3, 9, 2, 1);

    if let Ok(manufacturer) = String::from_utf8(manufacturer.stdout) {
        let manufacturer = manufacturer.trim();
        let l = Label::new(Some(&manufacturer));
        l.add_css_class("debug");
        grid.attach(&l, 5, 9, 2, 1);
    } else {
        let l = Label::new(Some("Unknown"));
        l.add_css_class("debug");
        grid.attach(&l, 5, 9, 2, 1);
    }

    //Software------------------------------------------
    let l = Label::new(Some("Software"));
    l.add_css_class("bold");
    grid.attach(&l, 3, 10, 4, 1);

    //Kernel version
    let l = Label::new(Some("Kernel: "));
    l.add_css_class("debug");
    grid.attach(&l, 3, 12, 2, 1);

    let os_version = system.kernel_version().unwrap();

    let l = Label::new(Some(&os_version));
    l.add_css_class("debug");
    grid.attach(&l, 5, 12, 2, 1);

    //Display server
    let l = Label::new(Some("Display server: "));
    l.add_css_class("debug");
    grid.attach(&l, 3, 13, 2, 1);

    let loginctl_output = Command::new("loginctl")
        .output()
        .expect("Failed to execute loginctl command")
        .stdout;

    let output = String::from_utf8(loginctl_output).unwrap();

    let session_id = output
        .split_whitespace()
        .find(|word| word.contains(['1', '2', '3', '4', '5', '6', '7', '8', '9', '0']))
        .unwrap_or_else(|| {
            eprintln!("No session ID found");
            ""
        });

    let session_type_output = Command::new("loginctl")
        .arg("show-session")
        .arg(session_id)
        .arg("-p")
        .arg("Type")
        .output()
        .expect("Failed to execute loginctl show-session command");

    let session_type = String::from_utf8_lossy(&session_type_output.stdout)
        .trim()
        .to_string();

    let session_type = session_type[5..].to_string();

    let l = Label::new(Some(&session_type));
    l.add_css_class("debug");
    grid.attach(&l, 5, 13, 2, 1);

    //Desktop
    let l = Label::new(Some("Desktop: "));
    l.add_css_class("debug");
    grid.attach(&l, 3, 14, 2, 1);

    let command = Command::new("env")
        .output()
        .expect("Failed to execute env command")
        .stdout;

    let output = String::from_utf8(command).unwrap();

    let desktop_session_line = output
        .lines()
        .find(|line| line.contains("DESKTOP_SESSION="))
        .expect("Failed to find DESKTOP_SESSION line");

    let desktop_session_value = desktop_session_line
        .split('=')
        .nth(1)
        .expect("Failed to extract DESKTOP_SESSION value");

    let l = Label::new(Some(&desktop_session_value));
    l.add_css_class("debug");
    grid.attach(&l, 5, 14, 2, 1);

    add_empty_cells(&grid);


    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_policy(PolicyType::Automatic, PolicyType::Always);
    scrolled_window.set_child(Some(&grid));
    scrolled_window.set_hexpand(true);
    scrolled_window.set_vexpand(true);
    scrolled_window
}
