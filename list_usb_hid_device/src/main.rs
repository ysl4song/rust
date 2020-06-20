use winapi::um::setupapi::*;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::shared::hidsdi::*;
use winapi::shared::hidclass::GUID_DEVINTERFACE_HID;
use winapi::shared::minwindef::{TRUE, FALSE};
use std::ptr::null_mut;
use std::mem;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::io::Error;
use winapi::um::fileapi;
use winapi::um::winnt::{GENERIC_READ, GENERIC_WRITE, FILE_SHARE_READ, FILE_SHARE_WRITE, BOOLEAN};


unsafe fn u16_ptr_to_string(ptr: *const u16) -> String {
    let len = (0..).take_while(|&i| *ptr.offset(i) != 0).count();
    let slice = std::slice::from_raw_parts(ptr, len);

    OsString::from_wide(slice).to_string_lossy().into() 
}

fn from_wide_string(s: &[u16]) -> String 
{ 
	let slice = s.split(|&v| v == 0).next().unwrap(); 
	OsString::from_wide(slice).to_string_lossy().into() 
}

fn main() {
    
    unsafe {
        let guid = GUID_DEVINTERFACE_HID;
        let device_info_set : HDEVINFO;
        let mut index : u32 = 0;
        let mut required_size : u32 = 0;

        device_info_set = SetupDiGetClassDevsW(
            &guid, 
            null_mut(), 
            null_mut(), 
            DIGCF_PRESENT | DIGCF_DEVICEINTERFACE,
        );

        if device_info_set == INVALID_HANDLE_VALUE {
            println!("ERROR : Unable to enumerate device.\n");
            return;
        }

        let mut device_interface_data : SP_DEVICE_INTERFACE_DATA = mem::zeroed();
        device_interface_data.cbSize = mem::size_of::<SP_DEVICE_INTERFACE_DATA>() as u32;

        loop {
            let mut _complete = SetupDiEnumDeviceInterfaces(
                device_info_set,
                null_mut(),
                &guid, 
                index, 
                &mut device_interface_data,
            );

            if _complete == FALSE {
                break;
            }

            _complete = SetupDiGetDeviceInterfaceDetailW(
                device_info_set,
                &mut device_interface_data,
                null_mut(),
                0,
                &mut required_size,
                null_mut(),
            );

            if required_size == 0 {
                println!("ERROR : SetupDiGetDeviceInterfaceDetailW(1) failed: {}\n", Error::last_os_error());
			    break;
            }

            //println!("required_size = {} \n", required_size);
            
            let _p_buffer = libc::malloc(required_size as usize) as PSP_DEVICE_INTERFACE_DETAIL_DATA_W;
            let _buffer_size = mem::size_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_W>();
            libc::memset(_p_buffer as *mut core::ffi::c_void, 0, _buffer_size);
            (*_p_buffer).cbSize = _buffer_size as u32;

            let mut devinfo_data : SP_DEVINFO_DATA = mem::zeroed();
            devinfo_data.cbSize = mem::size_of::<SP_DEVINFO_DATA>() as u32;

            _complete = SetupDiGetDeviceInterfaceDetailW(
                device_info_set,
                &mut device_interface_data,
                _p_buffer,
                required_size,
                &mut required_size,
                &mut devinfo_data,
            );

            if _complete == FALSE {
			    println!("ERROR : SetupDiGetDeviceInterfaceDetailW(2) failed: {}\n", Error::last_os_error());
			    break;
            }

            let device_path = (*_p_buffer).DevicePath.as_ptr();
            let path = u16_ptr_to_string(device_path);

            println!("device path: {:#?} \n", path);

            let handle = fileapi::CreateFileW(
                device_path, 
                GENERIC_READ | GENERIC_WRITE, 
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                null_mut(),
                fileapi::OPEN_EXISTING, 
                0, 
                null_mut(),
            );

            if handle == INVALID_HANDLE_VALUE {
                println!("ERROR : Unable to CreateFile.\n");
                // break;
            }
            else {
                let mut attributes : HIDD_ATTRIBUTES = mem::zeroed();
                attributes.Size = mem::size_of::<SP_DEVICE_INTERFACE_DATA>() as u32;
                
                if  0 == HidD_GetAttributes(handle, &mut attributes) {
                    println!("ERROR : Unable to CreateFile.\n");
                }
                else {
                    println!("VID={:04X}, PID={:04X} \n", attributes.VendorID, attributes.ProductID);
                }
            }

            // release resource
            libc::free(_p_buffer as *mut core::ffi::c_void);

            index = index + 1;
        }

        SetupDiDestroyDeviceInfoList(device_info_set);
    }

    println!("\n----- END -----\n");
}
