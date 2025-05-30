use std::{env, fs, io::{self, stdout, Write}, path::Path, process::{self, Command}};

use crate::gb;

pub (in crate::core) fn get_device_size(dev: &str) -> Option<f64> {
    let mut size_gb: f64 = 0.0;

    if let Some(device_name) = dev.strip_prefix("/dev/") {
        let read_size = fs::read_to_string(format!("/sys/block/{}/size", device_name)).unwrap();
        let convert = read_size.trim().parse::<u64>().unwrap_or(0);
        if convert == 0 {
            return None;
        }

        size_gb = gb!(convert);
    }

    Some(size_gb)
}

pub(in crate::core) fn get_connected_devices() -> Vec<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("lsblk -p -S -o NAME,TRAN |grep usb |awk '{print $1}'")
        .output()
        .expect("Failed to get USB devices");

    String::from_utf8_lossy(&output.stdout).to_string()
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty()) 
        .collect()
}

pub(in crate::core) fn read_str_stdin() -> String { // Read string for confirmation
    let mut input = String::new();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

pub(in crate::core) fn read_int_stdin() -> i32 { // Read int for menu index access
    let mut input = String::new();

    print!("Drive> ");
    stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap_or(-1)
}

pub(in crate::core) fn ensure_destructiv_action(device: &str) -> bool {
    println!("WARNING: THIS IS A DESTRUCTIVE ACTION! ALL DATA WILL BE LOST!");
    print!("Type \"YES\" to wipe {}.. ", device);

    let confirm = read_str_stdin();
    confirm == "YES" || confirm == "yes"
}


pub(in crate::core) fn check_root() {
    // Ensure currend session is executed as root by 
    // checking the environment variable key
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

#[inline]
pub(in crate::core) fn device_exist(dev: &str) -> bool {
    Path::new(dev).exists()
}