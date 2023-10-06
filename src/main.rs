use std::{process::exit, time::Duration};
use std::io::{self};

use serialport::{available_ports, SerialPortType, SerialPortInfo};

fn main() {
    let vendors: Vec<u16> = vec![0x1eab,0xa108,0x28e9,0x0c2e,0x0483];
    let ports = available_ports().expect("No ports found!");
    let mut found: Vec<SerialPortInfo> = vec![];
    for ref p in ports {
        if !p.port_name.as_str().contains("tty") { continue; }
        match &p.port_type {
            SerialPortType::UsbPort(info) => {
                if !vendors.contains(&info.vid) { continue; }
                found.push(p.clone());
                
                println!("    Type: USB");
                println!("    VID:{:04x} PID:{:04x}", info.vid, info.pid);
                println!(
                    "     Serial Number: {}",
                    info.serial_number.as_ref().map_or("", String::as_str)
                );
                println!(
                    "      Manufacturer: {}",
                    info.manufacturer.as_ref().map_or("", String::as_str)
                );
                println!(
                    "           Product: {}",
                    info.product.as_ref().map_or("", String::as_str)

            );
            }            
            SerialPortType::BluetoothPort => {}            
            SerialPortType::PciPort => {}            
            SerialPortType::Unknown => {}
        }
    }
    if found.len() == 0 { exit(0); }
    println!("Found:");
    let port_info = found.first().unwrap();
    let baud_rate: u32 = 115200;
    println!("{}", port_info.port_name);

    let port = serialport::new(&port_info.port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open();

    match port {
        Ok(mut port) => {
            let mut serial_buf: Vec<u8> = vec![0; 1000];
            println!("Receiving data on {} at {} baud:", &port_info.port_name, &baud_rate);
            loop {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => {
                        let code = String::from_utf8_lossy(&serial_buf[..t]);
                        //                        io::stdout().write_all(&serial_buf[..t]).unwrap();
                        let path = format!("https://dev-crm.fsfera.ru/select_mdse/?crmbar_version=rcrmbar&barcode={}",code.trim());
                        match open::that(&path) {
                            Ok(()) => println!("Opened '{}' successfully.", path),
                            Err(err) => eprintln!("An error occurred when opening '{}': {}", path, err),
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_info.port_name, e);
            ::std::process::exit(1);
        }
    }
    
}

/*
fn valid_baud(val: &str) -> Result<(), String> {
    val.parse::<u32>()
        .map(|_| ())
        .map_err(|_| format!("Invalid baud rate '{}' specified", val))
}
*/
