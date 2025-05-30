use super::{dd, utils::{self, read_int_stdin, get_connected_devices, get_device_size}};
use crate::{core::utils::{device_exist, read_str_stdin}, printdec, tern};

use std::{env, io::{stdout, Write}, process::{exit, Command}};

fn show_options(devices: &Vec<String>) -> usize {
    printdec!('#', 30);
    println!("0 - Exit");

    let mut n = 0;

    for idx in 0..devices.len() {
        n = idx + 1;

        let device = &devices[idx];
        if let Some(device_size) =  get_device_size(&device) {
            println!("{n} - {} [Size: {:.2} GB]", device, device_size);
        } else {
            println!("{n} - {}", device);
        }
    }

    n += 1;

    println!("{n} - Specify device manually");
    printdec!('#', 30);

    n
}

#[derive(PartialEq)]
enum Mode {
    None,
    Zero,
    Random
}

impl Mode {
    fn new() -> Self {
        Self::None
    }

    fn set_mode(&mut self, mode: Mode) {
        *self = mode
    }

    fn get_file(&self) -> Option<&str> {
        match self {
            Mode::Random => Some("/dev/urandom"),
            Mode::Zero => Some("/dev/zero"),
            _ => None
        }
    }
}

fn session(file_by_mode: &str, mode_label: &str, bs: &str) {
    loop {
        let devices = get_connected_devices();

        println!("\nUSBWipe");
        printdec!('#', 30);
        println!("Mode: {}", mode_label);
        println!("Block Size: {}", tern!(bs == "40M", format!("{} [default]", bs), bs.to_string()));

        let last = show_options(&devices);
        let mut option = read_int_stdin();
        let mut device_count: i32 = devices.len().try_into().unwrap();

        device_count += 1; // Add for manual option

        if option > device_count || option < 0 {
            println!("Invalid input! Please choose from 0 - {}", device_count);
            continue;
        } else if option == 0 {
            println!("Exit!");
            break;
        }

        let device: String;

        if option == last.try_into().unwrap() { // Enter manual mode
            loop {
                let output = Command::new("lsblk").output().unwrap(); 
                println!("\n{}", String::from_utf8_lossy(&output.stdout));
                stdout().flush().unwrap();
                
                print!("Enter Device> ");

                let manual_dev = read_str_stdin();
                if manual_dev.is_empty() {
                    println!("Please enter a device!");
                    continue;
                }

                if !device_exist(&manual_dev) {
                    println!("Device {} does not exist!", manual_dev);
                    continue;
                }

                device = manual_dev;

                break;
            }
        } else {
            option -= 1;
            device = devices[option as usize].clone();
        }
        
        if device_exist(&device) && utils::ensure_destructiv_action(&device) {
            dd::wipe(&device, file_by_mode, bs);
        }
    }
}

pub(crate) fn start() {
    utils::check_root();

    let args: Vec<String> = env::args().collect();
    let option = match args.get(1) {
        Some(mode) => mode,
        None => {
            eprintln!("Please specify the mode!");
            eprintln!("Options:");
            eprintln!(" random [block size, default: 40MB] - fill drive with random data");
            eprintln!(" zero   [block size, default: 40MB] - fill drive with zeros");
            eprintln!("");
            eprintln!("Examples:");
            eprintln!(" sudo ./usbwipe random");
            eprintln!(" sudo ./usbwipe zero 40M");
            exit(-1);
        }
    };

    let block_size = args.get(2).map(|custom_bs| custom_bs.clone()).unwrap_or_else(|| "40M".to_string());

    let mut mode = Mode::None;

    if option == "random" {
        mode = Mode::Random;
    } else if option == "zero" {
        mode = Mode::Zero;
    }

    if mode == Mode::None {
        eprintln!("Invalid parameter: {}", option);
        exit(-1);
    }

    let mut session_mode = Mode::new();
    session_mode.set_mode(mode);
    
    if let Some(file) = session_mode.get_file() {
        session(file, option.as_str(), block_size.as_str());
    }
}