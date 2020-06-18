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


unsafe fn u16_ptr_to_string(ptr: *const u16) -> OsString {
    let len = (0..).take_while(|&i| *ptr.offset(i) != 0).count();
    let slice = std::slice::from_raw_parts(ptr, len);

    OsString::from_wide(slice)
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

            println!("required_size = {} \n", required_size);
            
            let _p_buffer = libc::malloc(required_size as usize) as PSP_DEVICE_INTERFACE_DETAIL_DATA_W;
            let _buffer_size = mem::size_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_W>();
            libc::memset(_p_buffer as *mut core::ffi::c_void, 0, _buffer_size);
            (*_p_buffer).cbSize = _buffer_size as u32;

            //println!("(*_p_buffer).cbSize = {} \n", (*_p_buffer));

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

            let device_path = (*_p_buffer).DevicePath;
            let device_path_ptr = device_path.to_vec().as_ptr();
            //let face_name_ptr = &device_path as &[u16];
            //let path = OsString::from_wide(&device_path);
            let path = u16_ptr_to_string(device_path_ptr);
            println!("device path: {:#?} \n", path);

            index = index + 1;
        }

        SetupDiDestroyDeviceInfoList(device_info_set);
    }

    println!("\n----- END -----\n");
}
