extern crate sys_info;
use chrono::Utc;
use std::env;
use std::process::exit;
use sys_info::{
    disk_info, hostname, linux_os_release, mem_info, os_release, DiskInfo, LinuxOSReleaseInfo,
    MemInfo,
};
mod custom;
use custom::*;

fn main() {
    //// HOST INFORMATION
    println!("-=-= HOST INFORMATION =-=-");
    println!("Hostname: {}", hostname().unwrap_or_default());
    println!("Date: {}", Utc::now());
    let release_info: LinuxOSReleaseInfo = match linux_os_release() {
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1)
        }
        Ok(release_info) => release_info,
    };

    println!(
        "OS: {} ({})",
        release_info.pretty_name.unwrap_or_default(),
        os_release().unwrap_or_default()
    );

    //// LINUX MEMORY INFORMATION
    println!("\n\n-=-= MEMORY INFORMATION =-=-");
    let mem_info: MemInfo = match mem_info() {
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1)
        }
        Ok(mem_info) => mem_info,
    };

    println!(
        "Total Memory: {} bytes\nFree Memory: {} bytes\nAvailable: {} bytes\nSwap: {} bytes",
        &mem_info.total, &mem_info.free, &mem_info.avail, &mem_info.swap_total
    );

    //// DISK INFORMATION
    println!("\n\n-=-= DISK INFORMATION =-=-");
    let disk_info: DiskInfo = match disk_info() {
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1)
        }
        Ok(disk_info) => disk_info,
    };

    println!(
        "Total Disk Space: {} bytes\nFree Disk Space: {} bytes",
        &disk_info.total, &disk_info.free
    );

    //// BATTER INFORMATION
    println!("\n\n-=-= BATTERY INFORMATION =-=-");
    println!(
        "Battery Capacity: {}\nBattery Status: {}",
        battery_amount(),
        battery_status()
    );

    //// ENVIRONMENT INFORMATION
    println!("\n\n-=-= ENVIRONMENT VARIABLES =-=-");
    for (var, val) in env::vars() {
        println!("{}={}", var, val);
    }

    //// EXTRA INFORMATION
    println!("\n\n-=-= OTHER INFORMATION =-=-");
    println!("Is ASLR Enabled? : {}", if aslr() { "Yes" } else { "No" });
    println!("PATH: {}", env::var("PATH").unwrap_or_default());
    println!("Docker? : {}", if in_docker() { "yes" } else { "No" });


    //// TIMERS, CRON JOBS, SERVICES
    println!("\n\n-=-= TIMERS, CRONJOBS, SERVICES =-=-");
    println!("Cron Jobs: {}\n\n", if crontab().is_empty() {String::from("No cronjobs for current user.")} else {crontab()} );
    println!("Timers: \n{}\n\n", timers());
    println!("Services: \n{}\n\n", sysd_services());
}
