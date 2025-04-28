/* 
* MIT License
* 
* Copyright (c) 2025 f42h
* 
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
* 
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
* 
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/

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
        // Preconstruct the final dd command
        UsbWipe {
            bin: "dd".to_string(),
            source: Self::construct_param("if=", "/dev/zero"),
            destination: Self::construct_param("of=", usb_device),
            bytes: Self::construct_param("bs=", "4M"), // 4MB block size
            status: Self::construct_param("status=", "progress"), // Display the wipe progress
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
        Ok(mut c) => c.wait().unwrap(), // Wait for dd to finish before call sync
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
