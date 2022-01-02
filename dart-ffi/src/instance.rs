use std::ops::Deref;
use ble_desktop::common::utils::*;
use ble_desktop::models::ble_core::{BleCore, BleRepo};
use crate::{runtime, utils::run_async};
use allo_isolate::Isolate;
use std::marker::{Sync, Send};
use futures::executor::block_on;
use futures::future::try_join_all;
use crate::utils::RUNTIME_THREAD;

struct BleCoreSend(*mut *const BleCore);

unsafe impl Send for BleCoreSend {}

unsafe impl Sync for BleCoreSend {}

#[no_mangle]
pub unsafe extern "C" fn ble_instance(
    ble: *mut *const BleCore
) {
    let ble_core_send = BleCoreSend(ble);

    run_async(move || {
        async {
            let ble_core_send = ble_core_send;
            BleCore::create().await;
            let instance = BleCore::get_instance();
            match instance {
                Some(ble_core) => {
                    ble_core_send.0.write(ble_core.as_ref());
                }
                _ => {
                    panic!("error to intantiate ble core")
                }
            }
        };
    });
}

#[no_mangle]
pub unsafe extern "C" fn get_list_devices(port: i64, seconds: u64) {
    let mut ble = BleCore::get_instance().unwrap().deref().clone();
    let rt = runtime!();
    rt.spawn(async move {
        let devices = (ble.list_devices(Some(seconds))).await;
        let json = map_device_to_json(devices);
        let isolate = Isolate::new(port);
        isolate.post(json);
    });
}