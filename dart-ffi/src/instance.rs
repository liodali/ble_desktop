use std::fmt::Error;
use std::ops::Deref;
use ble_desktop::common::utils::*;
use ble_desktop::models::ble_core::{BleCore, BleRepo};
use crate::{runtime, utils::run_async, error};
use allo_isolate::Isolate;
use futures::executor::block_on;
use std::sync::{Arc};
use std::marker::{Sync, Send};
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
            BleCore::create().await;
        };
    });
    let instance = BleCore::get_instance();
    match instance {
        Some(bleCore) => {
            ble_core_send.0.write(bleCore.as_ref());
        }
        _ => {
            panic!("error to intantiate ble core")
        }
    }
}

pub unsafe extern "C" fn get_list_devices(port: i64, seconds: u64) {
    let mut ble = BleCore::get_instance().unwrap().deref().clone();
    let rt = runtime!();
    rt.spawn(async move {
        let devices = (ble.list_devices(Some(seconds))).await;
        let isolate = Isolate::new(port);
        let json = map_device_to_json(devices);
        isolate.post(json);
    });
}