use winapi::um::setupapi::*;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::shared::hidsdi::*;
use winapi::shared::hidclass::GUID_DEVINTERFACE_HID;
use std::ptr::null_mut;
use std::mem;


// Get a win32 lpstr from a &str, converting u8 to u16 and appending '\0'
fn to_wstring(value: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;

    std::ffi::OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
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
            DIGCF_PRESENT | DIGCF_DEVICEINTERFACE);

        if device_info_set == INVALID_HANDLE_VALUE {
            println!("ERROR : Unable to enumerate device.\n");
            return;
        }

        let mut device_interface_data = SP_DEVICE_INTERFACE_DATA {
            cbSize: 0,
            InterfaceClassGuid: guid,
            Flags: 0,
            Reserved: 0,
        };
        device_interface_data.cbSize = mem::size_of::<SP_DEVICE_INTERFACE_DATA>() as u32;
        
        loop {
            let mut _complete = SetupDiEnumDeviceInterfaces(
                device_info_set,
                null_mut(),
                &guid, 
                index, 
                &mut device_interface_data);

            if _complete == 0 {
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

            if _complete == 0 {
			    println!("ERROR : SetupDiGetDeviceInterfaceDetailW fial.\n");
			    break;
            }
            
            //let mut pBuffer = Box::new(required_size);
            
            


            index = index + 1;
        }
    }



    println!("\n----- Hello, world! -----\n");
}
