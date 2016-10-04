#![allow(not_unsafe_ptr_arg_deref)]
#![feature(plugin)]
#![plugin(clippy)]

extern crate libc;

use libc::{c_char, c_int, c_uint, uint64_t, c_double, malloc, strncpy};
use std::{ffi, str, fmt};

// Return codes used by module during (un)initialization
pub const ZBX_MODULE_OK: c_int = 0;
pub const ZBX_MODULE_FAIL: c_int = -1;

// Module API versions
pub const ZBX_MODULE_API_VERSION: c_int = 2;
pub const ZBX_MODULE_API_VERSION_ONE: c_int = ZBX_MODULE_API_VERSION;

// Flags for commands
// Item does not accept parameters
pub const CF_NOPARAMS: c_uint = 0;

// Item accepts either optional or mandatory parameters
pub const CF_HAVEPARAMS: c_uint = 1;

// Item is defined in a loadable module
pub const CF_MODULE: c_uint = 2;

// Item is defined as user parameter
pub const CF_USERPARAMETER: c_uint = 4;

// Agent result types
pub const AR_UINT64: c_int = 1;
pub const AR_DOUBLE: c_int = 2;
pub const AR_STRING: c_int = 4;
pub const AR_TEXT: c_int = 8;
pub const AR_LOG: c_int = 16;
pub const AR_MESSAGE: c_int = 32;
pub const AR_META: c_int = 64;

// Return codes used by item callbacks
pub const SYSINFO_RET_OK: c_int = 0;
pub const SYSINFO_RET_FAIL: c_int = 1;

// Type used for creating new Zabbix item keys
#[derive(Debug)]
pub struct Metric {
    pub key: ffi::CString,
    pub flags: c_uint,
    pub function: extern "C" fn(*mut AGENT_REQUEST, *mut AGENT_RESULT) -> c_int,
    pub test_param: ffi::CString,
}
impl Metric {
    pub fn new(key: &str, flags: u32, function: extern "C" fn(*mut AGENT_REQUEST, *mut AGENT_RESULT) -> c_int, test_param: &str) -> Metric {
        Metric { key: ffi::CString::new(key).unwrap(), flags: flags as c_uint, function: function, test_param: ffi::CString::new(test_param).unwrap() }
    }

    pub fn to_zabbix_item(&self) -> ZBX_METRIC { ZBX_METRIC { key: self.key.as_ptr(), flags: self.flags as c_uint, function: self.function, test_param: self.test_param.as_ptr() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ZBX_HISTORY_FLOAT {
    pub itemid: uint64_t,
    pub clock: c_int,
    pub ns: c_int,
    pub value: f64,
}
impl Default for ZBX_HISTORY_FLOAT {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ZBX_HISTORY_INTEGER {
    pub itemid: uint64_t,
    pub clock: c_int,
    pub ns: c_int,
    pub value: uint64_t,
}
impl Default for ZBX_HISTORY_INTEGER {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ZBX_HISTORY_STRING {
    pub itemid: uint64_t,
    pub clock: c_int,
    pub ns: c_int,
    pub value: *const c_char,
}
impl Default for ZBX_HISTORY_STRING {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
impl fmt::Debug for ZBX_HISTORY_STRING {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "ZBX_HISTORY_STRING {{ itemid: {} clock: {} ns: {} value: {:?} ({:?}) }}", self.itemid, self.clock, self.ns, self.value, self.value()) }
}
impl ZBX_HISTORY_STRING {
    #![allow(dead_code)]
    pub fn value(self) -> Option<String> {
        match str::from_utf8(unsafe { ffi::CStr::from_ptr(self.value).to_bytes() }) {
            Ok(a) => Some(a.to_string()),
            Err(_) => None,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ZBX_HISTORY_TEXT {
    pub itemid: uint64_t,
    pub clock: c_int,
    pub ns: c_int,
    pub value: *const c_char,
}
impl Default for ZBX_HISTORY_TEXT {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
impl fmt::Debug for ZBX_HISTORY_TEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "ZBX_HISTORY_TEXT {{ itemid: {} clock: {} ns: {} value: {:?} ({:?}) }}", self.itemid, self.clock, self.ns, self.value, self.value()) }
}
impl ZBX_HISTORY_TEXT {
    #![allow(dead_code)]
    pub fn value(self) -> Option<String> {
        match str::from_utf8(unsafe { ffi::CStr::from_ptr(self.value).to_bytes() }) {
            Ok(a) => Some(a.to_string()),
            Err(_) => None,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ZBX_HISTORY_LOG {
    pub itemid: uint64_t,
    pub clock: c_int,
    pub ns: c_int,
    pub value: *const c_char,
    pub source: *const c_char,
    pub timestamp: c_int,
    pub logeventid: c_int,
    pub severity: c_int,
}
impl Default for ZBX_HISTORY_LOG {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
impl fmt::Debug for ZBX_HISTORY_LOG {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "ZBX_HISTORY_LOG {{ itemid: {} clock: {} ns: {} value: {:?} ({:?}) source: {:?} ({:?}) timestamp: {} logeventid: {} severity: {} }}",
               self.itemid,
               self.clock,
               self.ns,
               self.value,
               self.value(),
               self.source,
               self.source(),
               self.timestamp,
               self.logeventid,
               self.severity)
    }
}
impl ZBX_HISTORY_LOG {
    #![allow(dead_code)]
    pub fn value(self) -> Option<String> {
        match str::from_utf8(unsafe { ffi::CStr::from_ptr(self.value).to_bytes() }) {
            Ok(a) => Some(a.to_string()),
            Err(_) => None,
        }
    }
    pub fn source(self) -> Option<String> {
        match str::from_utf8(unsafe { ffi::CStr::from_ptr(self.source).to_bytes() }) {
            Ok(a) => Some(a.to_string()),
            Err(_) => None,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ZBX_HISTORY_WRITE_CBS {
    pub history_float_cb: Option<unsafe extern "C" fn(history: *const ZBX_HISTORY_FLOAT, history_num: c_int)>,
    pub history_integer_cb: Option<unsafe extern "C" fn(history: *const ZBX_HISTORY_INTEGER, history_num: c_int)>,
    pub history_string_cb: Option<unsafe extern "C" fn(history: *const ZBX_HISTORY_STRING, history_num: c_int)>,
    pub history_text_cb: Option<unsafe extern "C" fn(history: *const ZBX_HISTORY_TEXT, history_num: c_int)>,
    pub history_log_cb: Option<unsafe extern "C" fn(history: *const ZBX_HISTORY_LOG, history_num: c_int)>,
}
impl Default for ZBX_HISTORY_WRITE_CBS {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}


#[repr(C)]
#[derive(Debug)]
pub struct ZBX_METRIC {
    pub key: *const c_char,
    pub flags: c_uint,
    pub function: extern "C" fn(*mut AGENT_REQUEST, *mut AGENT_RESULT) -> c_int,
    pub test_param: *const c_char,
}

#[repr(C)]
#[derive(Debug)]
pub struct AGENT_REQUEST {
    key: *const c_char,
    nparam: c_int,
    params: *const *const c_char,
    lastlogsize: uint64_t,
    mtime: c_int,
}

impl AGENT_REQUEST {
    pub fn get_params<'a>(request: *mut AGENT_REQUEST) -> Vec<&'a [u8]> {
        unsafe {
            let len = (*request).nparam;
            let mut v = Vec::new();

            for i in 0..len {
                let ptr = (*request).params.offset(i as isize);
                let param = ffi::CStr::from_ptr(*ptr).to_bytes();
                v.push(param);
            }
            v
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct zbx_log_t {
    value: *const c_char,
    source: *const c_char,
    lastlogsize: uint64_t,
    timestamp: c_int,
    severity: c_int,
    logeventid: c_int,
    mtime: c_int,
}

#[repr(C)]
#[derive(Debug)]
pub struct AGENT_RESULT {
    _type: c_int,
    ui64: uint64_t,
    dbl: c_double,
    _str: *const c_char,
    text: *const c_char,
    msg: *const c_char,
    logs: *const *const zbx_log_t,
}

impl AGENT_RESULT {
    pub fn set_uint64_result(result: *mut AGENT_RESULT, value: u64) {
        unsafe {
            (*result)._type |= AR_UINT64;
            (*result).ui64 = value as uint64_t;
        }
    }

    pub fn set_f64_result(result: *mut AGENT_RESULT, value: f64) {
        unsafe {
            (*result)._type |= AR_DOUBLE;
            (*result).dbl = value as c_double;
        }
    }

    pub fn set_str_result(result: *mut AGENT_RESULT, value: &str) {
        unsafe {
            (*result)._type |= AR_STRING;
            (*result)._str = string_to_malloc_ptr(value);
        }
    }

    pub fn set_text_result(result: *mut AGENT_RESULT, value: &str) {
        unsafe {
            (*result)._type |= AR_TEXT;
            (*result).text = string_to_malloc_ptr(value);
        }
    }

    pub fn set_msg_result(result: *mut AGENT_RESULT, value: &str) {
        unsafe {
            (*result)._type |= AR_MESSAGE;
            (*result).msg = string_to_malloc_ptr(value);
        }
    }

    // TODO: Implement set_log_result(...)
}

// When the result of a Zabbix item is text (string, text and message)
// Zabbix expects to receive a pre-allocated pointer with the result
// string, which is free(3)'d by Zabbix once done with the result.
unsafe fn string_to_malloc_ptr(src: &str) -> *mut c_char {
    let c_src = ffi::CString::new(src).unwrap();
    let len = c_src.to_bytes_with_nul().len() as usize;

    let dst = malloc(len) as *mut c_char;
    strncpy(dst, c_src.as_ptr(), len);

    dst
}

pub fn create_items(metrics: &[Metric]) -> *const ZBX_METRIC {
    let items = metrics.iter().map(|metric| metric.to_zabbix_item()).collect::<Vec<_>>();
    items.as_ptr()
}
