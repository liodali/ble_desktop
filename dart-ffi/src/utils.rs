use once_cell::sync::Lazy;
use std::sync::Mutex;
use threadpool::{Builder, ThreadPool};
use lazy_static::lazy_static;
use std::{io};
use tokio::runtime::{Runtime};
use tokio::runtime::Builder as TokioBuilder;
use crate::lazy_static;

static THREAD_POOL: Lazy<Mutex<ThreadPool>> = Lazy::new(|| Mutex::new(Builder::new().build()));

pub fn run_async<F: FnOnce() + Send + 'static>(job: F) {
    THREAD_POOL.lock().unwrap().execute(job);
}

lazy_static! {
     pub static ref RUNTIME_THREAD: io::Result<Runtime> = TokioBuilder::new_multi_thread()
    .worker_threads(2)
    .enable_all()
    .thread_name("ble_async")
    .build();
}

#[macro_export]
macro_rules! error {
    ($result:expr) => {
        error!($result, 0);
    };
    ($result:expr, $error:expr) => {
        match $result {
            Ok(value) => value,
            Err(e) => {
                ffi_helpers::update_last_error(e);
                return $error;
            }
        }
    };
}
#[macro_export]
macro_rules! cstr {
    ($ptr:expr) => {
        cstr!($ptr, 0);
    };
    ($ptr:expr, $error:expr) => {{
        null_pointer_check!($ptr);
        error!(unsafe { CStr::from_ptr($ptr).to_str() }, $error)
    }};
}
#[macro_export]
macro_rules! runtime {
    () => {
        match crate::utils::RUNTIME_THREAD.as_ref() {
            Ok(rt) => rt,
            Err(_) => {
                return ;
            }
        }
    };
}
