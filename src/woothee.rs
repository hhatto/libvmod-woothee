extern crate libc;
extern crate woothee;
use libc::{c_int, c_uint, c_char, c_double, c_void, snprintf};

use std::ffi::CStr;
use std::ffi::CString;
use std::sync::Mutex;
use std::boxed::Box;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use woothee::{is_crawler, parser};

extern "C" {
    fn WS_Reserve(w: *const ws, b: c_uint) -> c_uint;
    fn WS_Release(w: *const ws, b: c_uint);
}

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

pub struct vrt_ctx {
    magic: c_uint,

    method: c_uint,
    handling: *const c_uint,

    msg: *const c_void,
    vsl: *const c_void,
    vcl: *const c_void,
    ws: *const ws,

    req: *const c_void,
    http_req: *const c_void,
    http_req_top: *const c_void,
    http_resp: *const c_void,

    bo: *const c_void,
    http_bereq: *const c_void,
    http_beresp: *const c_void,

    now: c_double,

    specific: *const c_void,
}

struct ws {
  magic: c_uint,
  id: [c_char; 4],
  s: *mut c_char,
  f: *mut c_char,
  r: *mut c_char,
  e: *mut c_char,
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
pub unsafe extern "C" fn vmod_parse_get_name(ctx: *const vrt_ctx, input: *const c_char) -> *const c_char {
    let ua = conv(input);
    let uaparser = parser::Parser::new();

    let u = WS_Reserve((*ctx).ws, 0);
    let p: *mut c_char = (*(*ctx).ws).f;
    match uaparser.parse(ua.as_str()) {
        Some(result) => {
            let l = result.name.len() + 1;
            let v = snprintf(p, l, CString::new(result.name).unwrap().as_ptr());
            WS_Release((*ctx).ws, (v+1) as u32);
            p
        }
        None => {
            WS_Release((*ctx).ws, 0);
            0 as (*const c_char)
        }
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
