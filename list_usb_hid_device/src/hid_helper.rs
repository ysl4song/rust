use winapi::um::
{
    setupapi::*,
    handleapi::{INVALID_HANDLE_VALUE, CloseHandle},
    fileapi,
    winnt::{GENERIC_READ, GENERIC_WRITE, FILE_SHARE_READ, FILE_SHARE_WRITE}
};
use winapi::shared::
{
    hidsdi::*, 
    hidclass::GUID_DEVINTERFACE_HID, 
    minwindef::FALSE, 
    guiddef
};
use std::ptr::null_mut;
use std::mem::size_of;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::io::Error;


const BUF_SIZE: usize = 512;

pub fn list_device() {
    let guid = GUID_DEVINTERFACE_HID;
    let device_info_set: HDEVINFO;
    let mut index: u32 = 0;
    let mut required_size : u32 = 0;
    let mut ptr_device_interface_detail_data: PSP_DEVICE_INTERFACE_DETAIL_DATA_W;
    
    // setup for HID class
    device_info_set = unsafe { SetupDiGetClassDevsW(
        &guid, 
        null_mut(), 
        null_mut(), 
        DIGCF_PRESENT | DIGCF_DEVICEINTERFACE,
    )};

    if device_info_set == INVALID_HANDLE_VALUE {
        println!("ERROR : Unable to enumerate device.\n");
        return;
    }

    let mut device_interface_data: Vec<SP_DEVICE_INTERFACE_DATA> = vec![SP_DEVICE_INTERFACE_DATA {
        cbSize: size_of::<SP_DEVICE_INTERFACE_DATA>() as u32,
        InterfaceClassGuid: guiddef::IID_NULL,
        Flags: 0,
        Reserved: 0,
    }];
    
    loop {
        // enumerate device
        let mut result = unsafe { SetupDiEnumDeviceInterfaces(
            device_info_set,
            null_mut(),
            &guid, 
            index, 
            device_interface_data.as_mut_ptr(),
        )};

        if result == FALSE {
            break;
        }

        // query needed buffer size
        unsafe { SetupDiGetDeviceInterfaceDetailW(
            device_info_set,
            device_interface_data.as_mut_ptr(),
            null_mut(),
            0,
            &mut required_size,
            null_mut(),
        )};

        if required_size == 0 {
            println!("ERROR : SetupDiGetDeviceInterfaceDetailW(1) failed: {}\n", Error::last_os_error());
            break;
        }

        //println!("required_size = {} \n", required_size);
        
        ptr_device_interface_detail_data = unsafe { libc::malloc(required_size as usize) as PSP_DEVICE_INTERFACE_DETAIL_DATA_W };
        unsafe { (*ptr_device_interface_detail_data).cbSize = size_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_W>() as u32 };

        let mut devinfo_data: Vec<SP_DEVINFO_DATA> = vec![SP_DEVINFO_DATA {
            cbSize: size_of::<SP_DEVINFO_DATA>() as u32,
            ClassGuid: guiddef::IID_NULL,
            DevInst: 0,
            Reserved: 0,
        }];

        // get the device path for device open
        result = unsafe { SetupDiGetDeviceInterfaceDetailW(
            device_info_set,
            device_interface_data.as_mut_ptr(),
            ptr_device_interface_detail_data,
            required_size,
            &mut required_size,
            devinfo_data.as_mut_ptr(),
        )};

        if result == FALSE {
            println!("ERROR : SetupDiGetDeviceInterfaceDetailW(2) failed: {}\n", Error::last_os_error());
            break;
        }

        // get the pointer to the device path array
        let device_path = unsafe { (*ptr_device_interface_detail_data).DevicePath.as_ptr() };
        //let path = unsafe { u16_ptr_to_string(device_path) };
        //println!("device path: {:#?} \n", path);

        query_hid_info(device_path);

        // release resource
        unsafe { libc::free(ptr_device_interface_detail_data as *mut core::ffi::c_void) };

        index = index + 1;
    }

    unsafe { SetupDiDestroyDeviceInfoList(device_info_set) };
}

fn query_hid_info(device_path: *const u16) {
    let handle = unsafe { fileapi::CreateFileW(
        device_path, 
        GENERIC_READ | GENERIC_WRITE, 
        FILE_SHARE_READ | FILE_SHARE_WRITE,
        null_mut(),
        fileapi::OPEN_EXISTING, 
        0, 
        null_mut(),
    )};

    if handle == INVALID_HANDLE_VALUE {
        //println!("ERROR : Unable to CreateFile. ({}) \n", Error::last_os_error());
    }
    else {
        println!("{{ \n");

        unsafe { println!("device path: {} \n", u16_ptr_to_string(device_path)) };

        let mut attributes: Vec<HIDD_ATTRIBUTES> = vec![HIDD_ATTRIBUTES {
            Size: 0,
            VendorID: 0,
            ProductID: 0,
            VersionNumber: 0,
        }];
        let ptr_attributes = attributes.as_mut_ptr();

        if  0 == unsafe { HidD_GetAttributes(handle, ptr_attributes) } {
            println!("ERROR : HidD_GetAttributes failed. ({}) \n", Error::last_os_error());
        }
        else {
            unsafe { println!("VID = {:04X}, PID = {:04X} \n", (*ptr_attributes).VendorID, (*ptr_attributes).ProductID) };
        }

        let mut buffer = [0u16; BUF_SIZE];

        if  0 == unsafe { HidD_GetProductString(handle, buffer.as_ptr() as *mut winapi::ctypes::c_void, BUF_SIZE as u32) } {
            println!("ERROR : HidD_GetProductString failed. ({}) \n", Error::last_os_error());
        }
        else {
            unsafe { println!("product_string = {} \n", u16_ptr_to_string(buffer.as_ptr())) };
        }

        if  0 == unsafe { HidD_GetSerialNumberString(handle, buffer.as_ptr() as *mut winapi::ctypes::c_void, BUF_SIZE as u32) } {
            println!("ERROR : HidD_GetSerialNumberString failed. ({}) \n", Error::last_os_error());
        }
        else {
            unsafe { println!("serial_number  = {} \n", u16_ptr_to_string(buffer.as_ptr())) };
        }

        if  0 == unsafe { HidD_GetManufacturerString(handle, buffer.as_ptr() as *mut winapi::ctypes::c_void, BUF_SIZE as u32) } {
            println!("ERROR : HidD_GetManufacturerString failed. ({}) \n", Error::last_os_error());
        }
        else {
            unsafe { println!("manufacture_string  = {} \n", u16_ptr_to_string(buffer.as_ptr())) };
        }

        unsafe { CloseHandle(handle) };

        println!("}}, \n");
    }
}

unsafe fn u16_ptr_to_string(ptr: *const u16) -> String {
    let len = (0..).take_while(|&i| *ptr.offset(i) != 0).count();
    let slice = std::slice::from_raw_parts(ptr, len);

    OsString::from_wide(slice).to_string_lossy().into() 
}