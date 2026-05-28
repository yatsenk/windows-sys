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

    println!("PID  |  Process Name  |  Virtual Address");
    for (pid, process) in system.processes() {
        let process_handle = unsafe {
            OpenProcess(
                PROCESS_ALL_ACCESS, 
                FALSE, 
                pid.as_u32()
            )
        };

        let base_address: *const c_void = std::ptr::null_mut();

        let result = unsafe {
            VirtualQueryEx(
                process_handle,
                base_address,
                &mut mbi,
                size_of::<MEMORY_BASIC_INFORMATION>(),
            )
        };

        if result != 0 {
            let base_address = base_address.wrapping_add(mbi.RegionSize);
            print!("[{}] {:?} {:?}\n", pid, process.name(), base_address);
        }
    }
}
