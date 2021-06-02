extern crate sys_info;
use std::process::exit;

use sys_info::{DiskInfo, LinuxOSReleaseInfo, MemInfo, disk_info, hostname, linux_os_release, mem_info, os_release};
use chrono::Utc;
fn main() {
    //// HOST INFORMATION
    println!("\n\n-=-= HOST INFORMATION =-=-");
    println!("Hostname: {}", hostname().unwrap_or_default());
    println!("Date: {}", Utc::now());


    //// OS RELEASE INFORMATION
    println!("\n\n-=-= RELEASE INFORMATION =-=-");
    let release_info: LinuxOSReleaseInfo = match linux_os_release() {
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1)
        }
        Ok(release_info) => release_info,
    };

    println!("{}: {}", release_info.pretty_name.unwrap_or_default(),
        os_release().unwrap_or_default()
    );


    //// LINUX MEMORY INFORMATION
    println!("\n\n-=-= MEMORY INFORMATION =-=-");
    let mem_info: MemInfo = match mem_info() {
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1)
        }
        Ok(mem_info) => mem_info
    };
    
    println!("Total Memory: {} bytes\nFree Memory: {} bytes\nAvailable: {} bytes\nSwap: {} bytes", &mem_info.total,
        &mem_info.free,
        &mem_info.avail,
        &mem_info.swap_total
    );


    //// DISK INFORMATION
    println!("\n\n-=-= DISK INFORMATION =-=-");
    let disk_info: DiskInfo = match disk_info() {
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1)
        }
        Ok(disk_info) => disk_info
    };
    
    println!("Total Disk Space: {} bytes\nFree Disk Space: {} bytes", &disk_info.total, &disk_info.free)
}
