mod setupapi;
mod hid;
mod common;

use std::ffi::c_void;
use std::mem::
{
    size_of, 
    MaybeUninit
};
use winapi::um::
{
    setupapi::SP_DEVINFO_DATA, 
    handleapi::{INVALID_HANDLE_VALUE, CloseHandle}
};


pub fn list_device() {
    let device_info_set = setupapi::get_device_info_set();
    let mut index = 0u32;

    if device_info_set == INVALID_HANDLE_VALUE {
        println!("ERROR : Unable to enumerate device.\n");
        return;
    }

    loop {
        let mut device_interface_data = match setupapi::get_device_interface(device_info_set, index) {
                Some(device_interface_data) => device_interface_data,
                None => break,
        };

        unsafe {
            let mut devinfo_data = MaybeUninit::<SP_DEVINFO_DATA>::zeroed();
            (*devinfo_data.as_mut_ptr()).cbSize = size_of::<SP_DEVINFO_DATA>() as u32;

            let device_interface_detail_data = setupapi::get_device_interface_detail_data(device_info_set, &mut device_interface_data, &mut (*devinfo_data.as_mut_ptr()));
            
            // get the pointer to the device path array
            let device_path = (*device_interface_detail_data).DevicePath.as_ptr();

            let pdo_name = setupapi::get_pdo_name(device_info_set, &mut devinfo_data.assume_init());

            
            let handle = hid::open_device(device_path);

            if handle != INVALID_HANDLE_VALUE {
                // print information
                println!("{{ \n");
                println!("device path: {} \n", common::u16_ptr_to_string(device_path));
                println!("pdo_name: {} \n", pdo_name);
                println!("dev_inst: {} \n", devinfo_data.assume_init().DevInst);
                hid::get_attributes(handle);
                hid::get_product_string(handle);
                hid::get_manufacturer_string(handle);
                hid::get_serial_number_string(handle);
                println!("}}, \n");

                CloseHandle(handle);
            }

            libc::free(device_interface_detail_data as *mut c_void);
        }
        
        index = index + 1;
    }

    setupapi::uninit(device_info_set);
}