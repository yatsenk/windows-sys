use std::ffi::{OsStr, c_void};
use sysinfo::System;
use itertools::Itertools;
use windows_sys::Win32::{
    Foundation::{CloseHandle, FALSE}, 
    System::{
        Memory::{MEMORY_BASIC_INFORMATION, VirtualQueryEx}, 
        Threading::{OpenProcess, PROCESS_QUERY_INFORMATION}
    }
};

fn main() { 
    get_all_virtual_addresses();
}

fn get_all_virtual_addresses() {
    let system = System::new_all();
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

    let process_handle = unsafe {
        OpenProcess(
            PROCESS_QUERY_INFORMATION, 
            FALSE, 
            pid
        )
    };

    let mut mbi = MEMORY_BASIC_INFORMATION::default();

    if process_handle != std::ptr::null_mut() {
        let mut base_address: *const c_void = std::ptr::null();

        loop {
            let result = unsafe {
                VirtualQueryEx(
                    process_handle,
                    base_address,
                    &mut mbi,
                    size_of::<MEMORY_BASIC_INFORMATION>(),
                )
            };

            if result == 0 { break; }
            
            print!("[{}] {:?} {:?}\n", pid, process_name, base_address);
            base_address = base_address.wrapping_add(mbi.RegionSize);
        }
        
        unsafe { CloseHandle(process_handle) };
    }
}
