extern crate sys_info;
use std::{mem, process::exit};

use sys_info::{linux_os_release, mem_info, LinuxOSReleaseInfo, MemInfo};

fn main() {
    //// OS RELEASE INFORMATION
    let release_info: LinuxOSReleaseInfo = match linux_os_release() {
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1)
        }
        Ok(release_info) => release_info,
    };

    println!("Os Release: {} {} release {} {}", &release_info.pretty_name.unwrap_or_default(),
        &release_info.build_id.unwrap_or_default(),
        &release_info.version.unwrap_or_default(),
        &release_info.cpe_name.unwrap_or_default()
    );


    //// LINUX MEMORY INFORMATION
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
}
