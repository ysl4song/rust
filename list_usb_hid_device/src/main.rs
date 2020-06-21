mod hid_helper;

// fn from_wide_string(s: &[u16]) -> String 
// { 
// 	let slice = s.split(|&v| v == 0).next().unwrap(); 
// 	OsString::from_wide(slice).to_string_lossy().into() 
// }

fn main() {
    hid_helper::list_device();
}
