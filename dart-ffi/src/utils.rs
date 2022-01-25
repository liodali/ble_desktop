use std::{ffi::CStr, io};
use std::os::raw::c_char;
use std::sync::Mutex;

use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use threadpool::{Builder, ThreadPool};
use tokio::runtime::Runtime;
use tokio::runtime::Builder as TokioBuilder;

use crate::lazy_static;

static THREAD_POOL: Lazy<Mutex<ThreadPool>> = Lazy::new(|| Mutex::new(Builder::new().build()));

pub fn run_async<F: FnOnce() + Send + 'static>(job: F) {
    THREAD_POOL.lock().unwrap().execute(job);
}

pub fn ptr_to_string(ptr: *const c_char) -> String {
    let result = unsafe {
        CStr::from_ptr(ptr).to_str()
    };
    match result {
        Ok(v) => {
            let value = String::from(v);
            return value;
        }
        Err(e) => {
            ffi_helpers::update_last_error(e);
            panic!("error to get data from dart")
        }
    }
}


lazy_static! {
     pub static ref RUNTIME_THREAD: io::Result<Runtime> = TokioBuilder::new_multi_thread()
    .worker_threads(2)
    .enable_all()
    .thread_name("ble_async")
    .build();
}

// #[macro_export]
// macro_rules! error {
//     ($result:expr) => {
//         error!($result, 0);
//     };
//     ($result:expr, $error:expr) => {
//         match $result {
//             Ok(value) => value,
//             Err(e) => {
//                 ffi_helpers::update_last_error(e);
//                 return $error;
//             }
//         }
//     };
// }

// #[macro_export]
// macro_rules! cstr {
//     ($ptr:expr) => {
//         cstr!($ptr, ())
//     };
//     ($ptr:expr, $error:expr) => {{
//         null_pointer_check!($ptr);
//         error!(unsafe { CStr::from_ptr($ptr).to_str() }, $error)
//     }};
// }
#[macro_export]
macro_rules! runtime {
    () => {
        match crate::utils::RUNTIME_THREAD.as_ref() {
            Ok(rt) => rt,
            Err(_) => {
                panic!("error")
            }
        }
    };
}
