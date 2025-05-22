# USBWipe

### Description
```
This application is simply a Rust based wrapper for the dd 
command to easily overwrite the device with either random data or zeros.
```

### Usage
- `Build` the project with the helper script [build.sh](https://github.com/f42h/usbwipe/blob/master/build.sh)
#### Note: The build script will automatically execute the application with `sudo`
```bash
bash build.sh
``` 

- Run the executable
```bash
sudo ./usbwipe
```

### Output
```
Please specify the mode!
Options:
 random [block size, default: 40MB] - fill drive with random data
 zero   [block size, default: 40MB] - fill drive with zeros

Examples:
 sudo ./usbwipe random
 sudo ./usbwipe zero 4M
```

### Usage Example
```
# sudo ./usbwipe random

USBWipe
###############################
Mode: random
Block Size: 40M [default]
###############################
0 - Exit
1 - /dev/sda [Size: 28.88 GB]
###############################
Drive> 1
WARNING: THIS IS A DESTRUCTIVE ACTION! ALL DATA WILL BE LOST!
Type "YES" to wipe /dev/sda.. YES # Confirm overwrite command
WRITING.. /dev/urandom -> /dev/sda
...
```