#!/bin/bash

echo
echo "Building the project.."
echo

cargo build --release && cp target/release/usbwipe .

echo
echo "Running executable.."
echo 

sudo ./usbwipe

echo
echo "To overwrite a USB device, execute: ./usbwipe <mode>"
echo "Example:"
echo -e "\tsudo ./usbwipe random"
echo