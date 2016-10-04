// Example Zabbix loadable module written in Rust
#![feature(plugin)]
#![plugin(clippy)]

extern crate zbx;
extern crate rand;

use rand::Rng;
use std::{str, slice};

#[no_mangle]
pub extern "C" fn zbx_module_api_version() -> i32 { zbx::ZBX_MODULE_API_VERSION_ONE }

#[no_mangle]
pub extern "C" fn zbx_module_init() -> i32 { zbx::ZBX_MODULE_OK }

#[no_mangle]
pub extern "C" fn zbx_module_uninit() -> i32 { zbx::ZBX_MODULE_OK }

#[no_mangle]
pub extern "C" fn zbx_module_item_list() -> *const zbx::ZBX_METRIC {
    let metrics = vec![
        zbx::Metric::new("rust.echo", zbx::CF_HAVEPARAMS, rust_echo, ""),
        zbx::Metric::new("rust.random", zbx::CF_NOPARAMS, rust_random, ""),
    ];

    zbx::create_items(&metrics)
}

#[no_mangle]
pub extern "C" fn rust_echo(request: *mut zbx::AGENT_REQUEST, result: *mut zbx::AGENT_RESULT) -> i32 {
    let params = zbx::AGENT_REQUEST::get_params(request);

    if params.len() != 1 {
        zbx::AGENT_RESULT::set_msg_result(result, "Invalid number of parameters");
        return zbx::SYSINFO_RET_FAIL;
    }

    let param = match str::from_utf8(params[0]) {
        Ok(p) => p,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    zbx::AGENT_RESULT::set_str_result(result, param);

    zbx::SYSINFO_RET_OK
}

#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn rust_random(request: *mut zbx::AGENT_REQUEST, result: *mut zbx::AGENT_RESULT) -> i32 {
    let mut rng = rand::thread_rng();
    let num = rng.gen::<u64>();

    zbx::AGENT_RESULT::set_uint64_result(result, num);

    zbx::SYSINFO_RET_OK
}


#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn zbx_module_history_write_cbs() -> zbx::ZBX_HISTORY_WRITE_CBS {
    zbx::ZBX_HISTORY_WRITE_CBS { history_float_cb: Some(dummy_history_float_cb),
                                 history_integer_cb: Some(dummy_history_integer_cb),
                                 history_string_cb: Some(dummy_history_string_cb),
                                 history_text_cb: Some(dummy_history_text_cb),
                                 history_log_cb: Some(dummy_history_log_cb), }
}

#[no_mangle]
#[allow(unused_variables)]
pub unsafe extern "C" fn dummy_history_float_cb(history: *const zbx::ZBX_HISTORY_FLOAT, history_num: i32) {
    if !history.is_null() && history_num > 1 {
        let histories = slice::from_raw_parts(history, history_num as usize);
        for h in histories {
            println!("dummy_history_float_cb: {:?}", histories);
        }
    } else {
        println!("dummy_history_float_cb: {:?}", *history);
    }
}

#[no_mangle]
#[allow(unused_variables)]
pub unsafe extern "C" fn dummy_history_integer_cb(history: *const zbx::ZBX_HISTORY_INTEGER, history_num: i32) {
    if !history.is_null() && history_num > 1 {
        let histories = slice::from_raw_parts(history, history_num as usize);
        for h in histories {
            println!("dummy_history_integer_cb: {:?}", histories);
        }
    } else {
        println!("dummy_history_integer_cb: {:?}", *history);
    }
}

#[no_mangle]
#[allow(unused_variables)]
pub unsafe extern "C" fn dummy_history_string_cb(history: *const zbx::ZBX_HISTORY_STRING, history_num: i32) {
    if !history.is_null() && history_num > 1 {
        let histories = slice::from_raw_parts(history, history_num as usize);
        for h in histories {
            println!("dummy_history_string_cb: {:?}", histories);
        }
    } else {
        println!("dummy_history_string_cb: {:?}", *history);
    }
}

#[no_mangle]
#[allow(unused_variables)]
pub unsafe extern "C" fn dummy_history_text_cb(history: *const zbx::ZBX_HISTORY_TEXT, history_num: i32) {
    if !history.is_null() && history_num > 1 {
        let histories = slice::from_raw_parts(history, history_num as usize);
        for h in histories {
            println!("dummy_history_text_cb: {:?}", histories);
        }
    } else {
        println!("dummy_history_text_cb: {:?}", *history);
    }
}

#[no_mangle]
#[allow(unused_variables)]
pub unsafe extern "C" fn dummy_history_log_cb(history: *const zbx::ZBX_HISTORY_LOG, history_num: i32) {
    if !history.is_null() && history_num > 1 {
        let histories = slice::from_raw_parts(history, history_num as usize);
        for h in histories {
            println!("dummy_history_log_cb: {:?}", histories);
        }
    } else {
        println!("dummy_history_log_cb: {:?}", *history);
    }
}
