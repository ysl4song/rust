use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;


pub unsafe fn u16_ptr_to_string(ptr: *const u16) -> String {
    let len = (0..).take_while(|&i| *ptr.offset(i) != 0).count();
    let slice = std::slice::from_raw_parts(ptr, len);

    OsString::from_wide(slice).to_string_lossy().into() 
}