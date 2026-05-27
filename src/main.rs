use std::ffi::OsStr;
use sysinfo::System;
use itertools::Itertools;
use windows_sys::Win32::{
    Foundation::{FALSE, GetLastError}, 
    System::{
        Memory::{MEMORY_BASIC_INFORMATION, VirtualQueryEx}, 
        Threading::{OpenProcess, PROCESS_ALL_ACCESS}
    }
};

fn main() { 
    let process_name = OsStr::new("explorer.exe");
    let system = System::new_all();
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

    let mut mbi = MEMORY_BASIC_INFORMATION::default();

    let hprocess = unsafe {
        OpenProcess(
            PROCESS_ALL_ACCESS, 
            FALSE, 
            pid
        )
    };

    unsafe {
        VirtualQueryEx(
            hprocess,
            std::ptr::null_mut(),
            &mut mbi,
            size_of::<MEMORY_BASIC_INFORMATION>(),
        )
    };

    println!("[ACCESS]");
    println!("Base Address: {:?}", mbi.BaseAddress);
    println!("Region Size: {:?}", mbi.RegionSize);

    let error = unsafe { GetLastError() };
    println!("[LAST CODE ERROR]: {}", error);
}
