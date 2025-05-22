use super::{dd, utils::{self, read_int_stdin, get_connected_devices, get_device_size}};
use crate::printdec;

use std::{env, process::exit};

fn show_options(devices: &Vec<String>) {
    printdec!('#', 30);
    
    println!("0 - Exit");
    for idx in 0..devices.len() {
        let n = idx + 1;
        let device = &devices[idx];
        if let Some(device_size) =  get_device_size(&device) {
            println!("{n} - {} [Size: {:.2} GB]", device, device_size);
        } else {
            println!("{n} - {}", device);
        }
    }

    printdec!('#', 30);
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

    fn get_file(&self) -> Option<String> {
        match self {
            Mode::Random => Some(String::from("/dev/urandom")),
            Mode::Zero => Some(String::from("/dev/zero")),
            _ => None
        }
    }
}

fn session(file_by_mode: String, mode_label: String) {
    loop {
        let devices = get_connected_devices();

        println!("\nUSBWipe - Mode: {}", mode_label);
        show_options(&devices);

        let mut option = read_int_stdin();
        let device_count:i32 = devices.len().try_into().unwrap();
        if option > device_count || option < 0 {
            println!("Invalid input! Please choose from 0 - {}", device_count);
            continue;
        } else if option == 0 {
            println!("Exit!");
            break;
        }
        
        option -= 1;

        let device = devices[option as usize].clone();

        if utils::ensure_destructiv_action(&device) {
            dd::wipe(&device, file_by_mode.clone());
        }
    }
}

pub(crate) fn start() {
    utils::check_root();

    let mut args = env::args();
    let option = match args.nth(1) {
        Some(mode) => mode,
        None => {
            eprintln!("Please specify the mode!");
            eprintln!("Options:");
            eprintln!(" random - fill drive with random data");
            eprintln!(" zero   - fill drive with zeros");
            exit(-1);
        }
    };

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
        session(file, option);
    }
}