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

pub struct vmod_priv<'a> {
    prv: *const Mutex<VmodWootheeResult<'a>>,
    len: c_int,
    free: *const c_void,
}

#[repr(C)]
pub struct vmod_woothee_result {
    pub name: *const c_char,
    pub parser: *const c_void,
}

pub struct VmodWootheeResult<'a> {
    wparser: parser::Parser<'a>,
    wresult: parser::WootheeResult<'a>,
}

#[no_mangle]
pub extern "C" fn init_function(_: *const c_void, prv: &mut vmod_priv, ev: VclEvent) -> c_int {
    match ev {
        VclEvent::Warm => {
            let w = VmodWootheeResult{
                wparser: parser::Parser::new(),
                wresult: parser::WootheeResult::new(),
            };
            let p = Mutex::new(w);
            prv.prv = Box::into_raw(Box::new(p));
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
pub unsafe extern "C" fn vmod_parser__init(_: *const c_void, result: *mut *const vmod_woothee_result, vcl_name: *const c_char) {
    //ALLOC_OBJ(result, 0xbbbbbbbb);
    //AN(result);

    let mut p = parser::Parser::new();
    let mut p_ptr: *mut c_void = &mut p as *mut _ as *mut c_void;
    *result = &vmod_woothee_result{name: CString::new("").unwrap().as_ptr(), parser: p_ptr};

    //let ua = conv(input);
    //match p.parse(ua.as_str()) {
    //    Some(r) => {
    //        let c_name = CString::new(r.name).unwrap();
    //    }
    //    None => {},
    //};
}

#[no_mangle]
pub unsafe extern "C" fn vmod_parser__fini(result: *mut *const vmod_woothee_result) {
}


#[no_mangle]
pub unsafe extern "C" fn vmod_parser_parse(_: *const c_void, result: *mut vmod_woothee_result, input: *const c_char) {
    let pparser = (*result).parser;
    let p: &mut parser::Parser = unsafe { &mut *(pparser as *mut parser::Parser) };
    let ua = conv(input);

    match p.parse(ua.as_str()) {
        Some(r) => {
            //let c_name = CString::new(r.name).unwrap();
            //(*result).name = c_name.as_ptr();
        }
        None => {},
    };
}

#[no_mangle]
pub unsafe extern "C" fn vmod_parser_get_name(_: *const c_void, result: *mut vmod_woothee_result) -> c_char {
    0
}
