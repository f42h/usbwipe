use std::{env, fs, process};
use std::process::{Command, exit};
use std::str;
use std::collections::HashMap;
use std::io::{self, stdout, Write};

use super::dd_wrapper::wipe;
use super::paths::Paths;

fn print_banner() {
    match fs::read_to_string(Paths::get_banner()) {
        Ok(banner) => println!("{}", banner),
        Err(_) => eprintln!("banner.txt not found")
    }
}

fn check_root() {
    match env::var("USER") {
        Ok(name) => {
            if name != "root" {
                eprintln!("Must be root...");
                process::exit(-1);
            }
        }
        Err(e) => {
            eprintln!("{:?}", e);
            process::exit(-1);
        }
    }
}

fn newline_char_count(result_string: &str) -> i32 {
    let mut count = 0; 

    for character in result_string.chars() {
        if character == '\n' {
            count += 1;
        }
    }

    count
}

fn block_devices_add(output_str: &str, delim: &str, bd_type: String) -> HashMap<String, String> {
    let mut block_devices: HashMap<String, String> = HashMap::new();
    let mut idx = 1;

    for token in output_str.split(delim) {
        if token.contains(bd_type.as_str()) {
            block_devices.insert(idx.to_string(), token.to_string());

            idx += 1;
        }
    }

    if idx == 1 {
        eprintln!("Could not find any SCSI disk devices!\n");
        exit(-1);
    }

    block_devices
}

fn get_block_devices() -> HashMap<String, String> {
    let output = Command::new("ls")
        .arg("/dev/")
        .output()
        .unwrap();

    let output_str = str::from_utf8(&output.stdout).unwrap();
    let delim = if newline_char_count(output_str) > 2 { "\n" } else { " " };

    block_devices_add(output_str, delim, "sd".to_owned())
}


fn read_int_stdio() -> i32 {
    let mut input = String::new();

    println!();
    print!("Drive> ");

    stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse::<i32>() {
        Ok(result) => result,
        Err(_) => -1,
    }
}

fn read_str_stdio() -> String {
    let mut input = String::new();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

struct DeviceMap<'a> {
    map: &'a HashMap<String, String>,
    size: usize,
}

impl<'a> DeviceMap<'a> {
    fn new(map: &'a HashMap<String, String>) -> Self {
        DeviceMap { map, size: map.len() }
    }
}

fn show_devices(device_map: &DeviceMap) {
    for n in 0..device_map.size {
        let idx = n.to_string();

        if let Some(device) = device_map.map.get(&idx) {
            println!("{n}] {}", device);
        }
    }
}

fn show_options(devices: &HashMap<String, String>) {
    println!("0] Quit");

    let device_map = DeviceMap::new(devices);
    show_devices(&device_map);
}

fn ensure_destructiv_action(device: &String) -> bool {
    print!("WARNING: THIS IS A DESTRUCTIVE ACTION! Type \"YES\" to wipe {}.. ", device);

    let confirm = read_str_stdio();
    if confirm == "YES" || confirm == "yes" {
        true
    } else {
        false
    }
}

pub fn menu_loop() {
    print_banner();
    check_root();

    loop {
        let devices = get_block_devices();
        let map_size = devices.keys().len() + 1;
    
        show_options(&devices);

        let device_idx = read_int_stdio();
        if device_idx == 0 {
            println!("Quitting UsbWipe!");
            process::exit(0);
        } else if device_idx == -1 || device_idx > map_size.try_into().unwrap() {
            eprintln!("Invalid input..");
            continue;
        }
    
        if device_idx > 0 && device_idx < map_size.try_into().unwrap() {
            let device = format!("/dev/{}", devices[&device_idx.to_string()]);
            
            if ensure_destructiv_action(&device) {
                wipe(&device);
            }
        }
    }
}

