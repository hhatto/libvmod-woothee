extern crate libc;
extern crate woothee;
use libc::{c_int, c_uint, c_char, c_void};

use std::ffi::CStr;
use std::ffi::CString;
use std::sync::Mutex;
use std::boxed::Box;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use woothee::{is_crawler, parser};

#[repr(C)]
pub enum VclEvent {
    Load = 0,
    Warm,
    Use,
    Cold,
    Discard,
}

pub struct vmod_priv {
    prv: *const Mutex<HashMap<String, c_int>>,
    len: c_int,
    free: *const c_void,
}

#[no_mangle]
pub extern "C" fn init_function(_: *const c_void, prv: &mut vmod_priv, ev: VclEvent) -> c_int {
    match ev {
        VclEvent::Warm => {
            let hash = Mutex::new(HashMap::new());
            prv.prv = Box::into_raw(Box::new(hash));
        }
        VclEvent::Cold => unsafe {
            Box::from_raw(prv);
        },
        _ => (),
    }
    0
}

fn conv(input: *const c_char) -> String {
    unsafe { CString::new(String::new() + CStr::from_ptr(input).to_str().unwrap()) }
        .unwrap()
        .into_string()
        .unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn vmod_is_crawler(_: *const c_void, input: *const c_char) -> c_uint {
    let ua = conv(input);

    match is_crawler(ua.as_str()) {
        true => return 1,
        false => return 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn vmod_parse(_: *const c_void,
                                    prv: &vmod_priv,
                                    input: *const c_char)
                                    -> c_uint {
    let ua = conv(input);
    let p = parser::Parser::new();

    match p.parse(ua.as_str()) {
        Some(result) => {
            if result.category == "crawler" {
                return 1;
            }
            return 0;
        }
        None => return 0,
    }
}
