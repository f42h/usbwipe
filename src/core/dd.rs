use std::process::{exit, Command};

struct UsbNull {
    bin: String,
    source: String,
    destination: String,
    bytes: String,
    status: String,
    convert: String,
}

impl UsbNull {
    fn new(usb_device: &str, file: String) -> Self {
        UsbNull {
            bin: "dd".to_string(),
            source: Self::construct_param("if=", &file), // Either /dev/urandom or /dev/zero
            destination: Self::construct_param("of=", usb_device), // Target
            bytes: Self::construct_param("bs=", "40M"), // 40MB block size
            status: Self::construct_param("status=", "progress"), // Display progress
            convert: Self::construct_param("conv=", "fsync") // Flush written data
        }
    }

    #[inline]
    fn construct_param(flag: &str, value: &str) -> String {
        flag.to_owned() + value 
    }

    fn get_command_wipe(&self) -> Command {
        // dd if=/dev/zero of=/dev/sda bs=4M status=progress conv=fsync
        let mut cmd = Command::new(&self.bin);
        
        cmd.arg(&self.source)
            .arg(&self.destination)
            .arg(&self.bytes)
            .arg(&self.status)
            .arg(&self.convert);

        cmd
    }

    fn get_command_sync(&self) -> Command {
        Command::new("sync")
    }
}

pub(crate) fn wipe(usb_device: &str, file: String) {
    let session = UsbNull::new(usb_device, file.clone());
    let mut cmd = session.get_command_wipe();

    println!("WRITING.. {} -> {}", file, usb_device);

    match cmd.spawn() {
        Ok(mut c) => c.wait().unwrap(), // Wait for dd to finish before call sync
        Err(err) => {
            eprintln!("Wipe command failed: {:?}", err);
            exit(-1);
        }
    };

    println!("Synchronising cache..");

    cmd = session.get_command_sync();
    if let Err(err) = cmd.spawn() {
        eprintln!("Sync command failed: {:?}", err);
        exit(-1);
    }

    println!("OK");
}