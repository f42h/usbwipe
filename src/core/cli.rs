use super::{dd, utils::{self, read_int_stdin, get_connected_devices, get_device_size}};
use crate::{printdec, tern};

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
            eprintln!(" sudo ./usbwipe zero 4M");
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