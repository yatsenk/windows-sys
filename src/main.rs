use std::ffi::{OsStr, c_void};
use sysinfo::System;
use itertools::Itertools;
use windows_sys::Win32::{
    Foundation::FALSE, 
    System::{
        Memory::{MEMORY_BASIC_INFORMATION, VirtualQueryEx}, 
        Threading::{OpenProcess, PROCESS_ALL_ACCESS}
    }
};

fn main() { 
    let system = System::new_all();
    let mut mbi = MEMORY_BASIC_INFORMATION::default();
    let process_name = OsStr::new("explorer.exe");

    let pid = match system.processes_by_exact_name(process_name).at_most_one() {
        Ok(Some(process)) => process.pid().as_u32(),
        Ok(None) => {
            println!("No process '{:?}' was found.", process_name);
            std::process::exit(1);
        }
        Err(_) => {
            println!("More than one process '{:?}' was found.", process_name);
            std::process::exit(1);
        }
    };

    println!("[{pid}] {:?}", process_name);

    let process_handle = unsafe {
        OpenProcess(
            PROCESS_ALL_ACCESS, 
            FALSE, 
            pid
        )
    };

    let base_address: *const c_void = std::ptr::null_mut();
    
    println!("Base Address:       {:?}", base_address);
    println!("\n---------- Begin Memory Search ----------");

    let result = unsafe {
        VirtualQueryEx(
            process_handle,
            base_address,
            &mut mbi,
            size_of::<MEMORY_BASIC_INFORMATION>(),
        )
    };

    while result != 0 {
        let base_address = base_address.wrapping_add(mbi.RegionSize);
        println!("Base Address:       {:?}", base_address);
    }

    println!("\n---------- End Memory Search ----------");
    println!("Base Address:       {:?}", base_address);
}
