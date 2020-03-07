use std::ffi::CStr;
use widestring::U16CString;
mod textractor_ws;

fn get_property(info_array: *const InfoForExtension, property_name: &str) -> i64 {
    let mut p = info_array;
    while !p.is_null() {
        unsafe {
            let p_name = (*p).name;
            let cs1 = CStr::from_ptr(p_name).to_str().unwrap();
            let cs2 = property_name;
            if cs1 == cs2 {
                let v = (*p).value;
                return v;
            }
            p = p.add(1);
        }
    }
    println!("Could not find property {}", property_name);
    panic!("Could not find property");
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InfoForExtension {
    pub name: *mut ::std::os::raw::c_char,
    pub value: i64,
}

#[no_mangle]
pub extern "C" fn OnNewSentence(
    sentence: *const u16,
    sentence_info: *const InfoForExtension,
) -> *const u16 {
    let u16_str: U16CString;
    unsafe {
        u16_str = U16CString::from_ptr_str(sentence);
    }

    let safe_s = u16_str.to_string_lossy();
    let current_select = get_property(sentence_info, "current select");
    let text_number = get_property(sentence_info, "text number");
    if current_select != 0 && text_number > 1 {
        textractor_ws::handle(safe_s);
    }
    sentence
}
