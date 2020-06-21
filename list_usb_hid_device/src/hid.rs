use winapi::um::
{
    fileapi,
    winnt::{HANDLE, GENERIC_READ, GENERIC_WRITE, FILE_SHARE_READ, FILE_SHARE_WRITE}
};
use winapi::shared::hidsdi::*;
use std::ptr::null_mut;
use std::io::Error;
use std::mem::MaybeUninit;

use super::common;

const BUF_SIZE: usize = 512;

pub fn open_device(device_path: *const u16) -> HANDLE {
    let handle = unsafe { fileapi::CreateFileW(
        device_path, 
        GENERIC_READ | GENERIC_WRITE, 
        FILE_SHARE_READ | FILE_SHARE_WRITE,
        null_mut(),
        fileapi::OPEN_EXISTING, 
        0, 
        null_mut(),
    )};

    handle
}

pub fn get_attributes(handle: HANDLE) {
    let mut attributes = MaybeUninit::<HIDD_ATTRIBUTES>::zeroed();

    if  0 == unsafe { HidD_GetAttributes(handle, attributes.as_mut_ptr()) } {
        println!("ERROR : HidD_GetAttributes failed. ({}) \n", Error::last_os_error());
    }
    else {
        unsafe { println!("VID = {:04X}, PID = {:04X} \n", attributes.assume_init().VendorID, attributes.assume_init().ProductID) };
    }
}

pub fn get_product_string(handle: HANDLE) {
    let buffer = [0u16; BUF_SIZE];

    if  0 == unsafe { HidD_GetProductString(handle, buffer.as_ptr() as *mut winapi::ctypes::c_void, BUF_SIZE as u32) } {
        println!("ERROR : HidD_GetProductString failed. ({}) \n", Error::last_os_error());
    }
    else {
        unsafe { println!("product_string = {} \n", common::u16_ptr_to_string(buffer.as_ptr())) };
    }
}

pub fn get_manufacturer_string(handle: HANDLE) {
    let buffer = [0u16; BUF_SIZE];

    if  0 == unsafe { HidD_GetManufacturerString(handle, buffer.as_ptr() as *mut winapi::ctypes::c_void, BUF_SIZE as u32) } {
        println!("ERROR : HidD_GetManufacturerString failed. ({}) \n", Error::last_os_error());
    }
    else {
        unsafe { println!("manufacture_string = {} \n", common::u16_ptr_to_string(buffer.as_ptr())) };
    }
}

pub fn get_serial_number_string(handle: HANDLE) {
    let buffer = [0u16; BUF_SIZE];

    if  0 == unsafe { HidD_GetSerialNumberString(handle, buffer.as_ptr() as *mut winapi::ctypes::c_void, BUF_SIZE as u32) } {
        println!("ERROR : HidD_GetSerialNumberString failed. ({}) \n", Error::last_os_error());
    }
    else {
        unsafe { println!("serial_number = {} \n", common::u16_ptr_to_string(buffer.as_ptr())) };
    }
}
