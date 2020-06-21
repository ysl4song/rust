use winapi::um::
{
    setupapi::*,
    handleapi::INVALID_HANDLE_VALUE,
};
use winapi::shared::
{
    hidclass::GUID_DEVINTERFACE_HID, 
    minwindef::FALSE, 
};
use std::ptr::null_mut;
use std::mem::{size_of, MaybeUninit};
use std::io::Error;

use super::common;

pub fn get_device_info_set() -> HDEVINFO {
    // setup for HID class
    let device_info_set = unsafe { SetupDiGetClassDevsW(
        &GUID_DEVINTERFACE_HID, 
        null_mut(), 
        null_mut(), 
        DIGCF_PRESENT | DIGCF_DEVICEINTERFACE,
    )};

    if device_info_set == INVALID_HANDLE_VALUE {
        println!("ERROR : SetupDiGetClassDevsW failed: {}\n", Error::last_os_error());
    }

    device_info_set
}

pub fn get_device_interface(
    device_info_set:HDEVINFO, 
    index: u32
) -> Option<SP_DEVICE_INTERFACE_DATA> {
    let mut device_interface_data = MaybeUninit::<SP_DEVICE_INTERFACE_DATA>::zeroed();

    unsafe {
        (*device_interface_data.as_mut_ptr()).cbSize = size_of::<SP_DEVICE_INTERFACE_DATA>() as u32;

        let result = SetupDiEnumDeviceInterfaces(
            device_info_set,
            null_mut(),
            &GUID_DEVINTERFACE_HID, 
            index, 
            device_interface_data.as_mut_ptr(),
        );

        if result == FALSE {
            return None;
        }

        Some(device_interface_data.assume_init())
    }
}

pub fn get_device_interface_detail_data(
    device_info_set: HDEVINFO,
    device_interface_data: &mut SP_DEVICE_INTERFACE_DATA,
    devinfo_data: &mut SP_DEVINFO_DATA
) -> PSP_DEVICE_INTERFACE_DETAIL_DATA_W {
    let mut required_size : u32 = 0;
    let mut ptr_device_interface_detail_data: PSP_DEVICE_INTERFACE_DETAIL_DATA_W;

    unsafe {
        // query needed buffer size
        SetupDiGetDeviceInterfaceDetailW(
            device_info_set,
            device_interface_data,
            null_mut(),
            0,
            &mut required_size,
            null_mut(),
        );

        if required_size == 0 {
            println!("ERROR : SetupDiGetDeviceInterfaceDetailW(1) failed: {}\n", Error::last_os_error());
        }

        //println!("required_size = {} \n", required_size);
        
        ptr_device_interface_detail_data = libc::malloc(required_size as usize) as PSP_DEVICE_INTERFACE_DETAIL_DATA_W;
        (*ptr_device_interface_detail_data).cbSize = size_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_W>() as u32;

        // let mut devinfo_data = MaybeUninit::<SP_DEVINFO_DATA>::zeroed();
        // (*devinfo_data.as_mut_ptr()).cbSize = size_of::<SP_DEVINFO_DATA>() as u32;

        // get the device path for device open
        let result = SetupDiGetDeviceInterfaceDetailW(
            device_info_set,
            device_interface_data,
            ptr_device_interface_detail_data,
            required_size,
            &mut required_size,
            devinfo_data,
        );

        if result == FALSE {
            println!("ERROR : SetupDiGetDeviceInterfaceDetailW(2) failed: {}\n", Error::last_os_error());
        }
    }

    ptr_device_interface_detail_data
}

pub fn get_pdo_name(
    device_info_set: HDEVINFO,
    devinfo_data: &mut SP_DEVINFO_DATA
) -> String {
    const BUFFER_SIZE: usize = 512;
    let buffer = [0u16; BUFFER_SIZE];
    let mut result = String::from("");

    unsafe {
        if 0 == SetupDiGetDeviceRegistryPropertyW(
            device_info_set,
            devinfo_data,
            SPDRP_PHYSICAL_DEVICE_OBJECT_NAME,
            &mut 0,
            buffer.as_ptr() as *mut u8,
            BUFFER_SIZE as u32,
            null_mut(),
        ) {
            println!("ERROR : SetupDiGetDeviceRegistryPropertyW failed. ({}) \n", Error::last_os_error());
        }
        else {
            result = common::u16_ptr_to_string(buffer.as_ptr());
        }

        result
    }
}

pub fn uninit(device_info_set: HDEVINFO) {
    unsafe { SetupDiDestroyDeviceInfoList(device_info_set) };
}
