use std::borrow::BorrowMut;
use std::future::Future;
use std::marker::{Send, Sync};
use std::os::raw::c_char;

use allo_isolate::Isolate;
use futures::executor::block_on;

use ble_desktop::common::utils::*;
use ble_desktop::models::ble_core::{BleCore, BleRepo};
use ble_desktop::models::filter_device::{FilterBleDevice, FilterType};

use crate::runtime;
use crate::utils::run_async;

struct BleCoreSend(*mut *const BleCore);

unsafe impl Send for BleCoreSend {}

unsafe impl Sync for BleCoreSend {}

#[no_mangle]
pub unsafe extern "C" fn ble_instance(
    ble: *mut *const BleCore,
    port: i64,
) {
    let ble_core_send = BleCoreSend(ble);
    let rt = runtime!();

    rt.block_on(async move {
        let ble_core_send = ble_core_send;

        let instance = BleCore::create().unwrap();
        ble_core_send.0.write(instance.as_ref());
        Isolate::new(port).post({
            println!();
            1
        });
    });
}

#[no_mangle]
pub unsafe extern "C" fn select_default_adapter(ble: *mut *const BleCore, port: i64) {
    let ble_core = BleCoreSend(ble);
    let rt = runtime!();
    rt.spawn(async move {
        let ble_core = ble_core;

        let mut ble_instance = ble_core.0.read().read();
        let adapts = ble_instance.get_adapters().unwrap();
        let mut iters_adapts = adapts.into_iter();
        let adapt = iters_adapts.nth(0).unwrap();
        ble_instance.set_adapter(&adapt);
        ble_core.0.write(&ble_instance);

        // let ble_instance = ble_core.0.read().as_ref().unwrap().clone();
        //  let mut bleC = BleCore::get_instance().unwrap().deref().clone();
        match ble_instance.get_adapter().is_none() {
            false => {
                Isolate::new(port).post({
                    1
                });
            }
            _ => {
                Isolate::new(port).post({
                    -1
                });
            }
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn get_list_devices(ble: *mut *const BleCore, port: i64, seconds: u64) {
    let ble_core = BleCoreSend(ble);
    let rt = runtime!();
    rt.spawn(async move {
        let ble_core = ble_core;
        let mut instance = ble_core.0.read().read();
        match instance.get_adapter().is_none() {
            /*Some(adpt)*/ false => {
                println!("one adapter was selected");
            }
            _ => {
                println!("no adapter was selected");
            }
        }
        let devices = instance.list_devices(Some(seconds), None);
        block_on(async {
            Isolate::new(port).task(
                async {
                    map_device_to_json(devices)
                }
            ).await;
        })
    });
}

#[no_mangle]
pub unsafe extern "C" fn connect_to_device(ble: *mut *const BleCore, port: i64, address: *const c_char) {
    let ble_core = BleCoreSend(ble);
    let adrDevice = address.as_ref().unwrap().to_string();
    let rt = runtime!();
    rt.spawn(async move {
        let ble_core = ble_core;
        let mut instance = ble_core.0.read().read();
        instance.connect(FilterBleDevice {
            name: FilterType::byAdr,
            value: adrDevice.to_string(),
        })
    });
}