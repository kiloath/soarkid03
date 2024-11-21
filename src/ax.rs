use std::ffi::c_char;
use encoding_rs::BIG5;

#[no_mangle]
pub extern "C" fn big5_to_utf8(big5: *const c_char) -> *const c_char {
    let c_str = unsafe { std::ffi::CStr::from_ptr(big5) };
    let big5_bytes = c_str.to_bytes();
    let (utf8_string, _, _) = BIG5.decode(big5_bytes);
    let utf8_c_string = std::ffi::CString::new(utf8_string.as_ref()).unwrap();
    utf8_c_string.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn free_string(ptr: *const c_char) {
    if ptr.is_null() {
        return;
    }
    let _ = std::ffi::CString::from_raw(ptr as *mut _);
}
