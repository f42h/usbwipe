use std::process::{exit, Command};

struct UsbWipe {
    bin: String,
    source: String,
    destination: String,
    bytes: String,
    status: String,
    convert: String,
}

impl UsbWipe {
    fn new(usb_device: &str) -> Self {
        UsbWipe {
            bin: "dd".to_string(),
            source: Self::construct_param("if=", "/dev/zero"),
            destination: Self::construct_param("of=", usb_device),
            bytes: Self::construct_param("bs=", "4M"), 
            status: Self::construct_param("status=", "progress"),
            convert: Self::construct_param("conv=", "fsync"),
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

    fn get_command_sync() -> Command {
        Command::new("sync")
    }
}

pub fn wipe(usb_device: &str) {
    let usb_wipe = UsbWipe::new(usb_device);
    let mut cmd = usb_wipe.get_command_wipe();

    println!("Wiping: {}..", usb_device);
    let child = cmd.spawn();
    match child {
        Ok(mut c) => c.wait().unwrap(),
        Err(e) => {
            eprintln!("Wipe command failed: {:?}", e);
            exit(-1);
        }
    };

    println!("Synchronising cache..");
    cmd = UsbWipe::get_command_sync();
    if let Err(e) = cmd.spawn() {
        eprintln!("Sync command failed: {}", e);
        exit(-1);
    }
}
