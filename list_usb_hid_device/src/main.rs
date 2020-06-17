use winapi::um::setupapi::*;
use std::ptr::null_mut;
use winapi::DEFINE_DEVPROPKEY;
use winapi::shared::devpropdef::DEVPROPKEY;

DEFINE_DEVPROPKEY!(DEVPKEY_Device_Manufacturer, 0xa45c254e, 0xdf1c, 0x4efd, 0x80, 0x20, 0x67, 0xd1, 0x46, 0xa8, 0x50, 0xe0, 13);

// Get a win32 lpstr from a &str, converting u8 to u16 and appending '\0'
fn to_wstring(value: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;

    std::ffi::OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

fn main() {
    
    let mut deviceInfoSet : HDEVINFO;

    unsafe {
        deviceInfoSet = SetupDiGetClassDevsW(null_mut(), to_wstring("HID").as_ptr(), null_mut(), DIGCF_ALLCLASSES | DIGCF_PRESENT);


    }

    println!("Hello, world!");
}
