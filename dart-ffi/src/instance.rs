use std::marker::{Send, Sync};
use std::mem;
use std::os::raw::c_char;

use allo_isolate::Isolate;
use futures::executor::block_on;

use ble_desktop::common::utils::*;
use ble_desktop::models::ble_core::{BleCore, BleRepo};
use ble_desktop::models::filter_device::{FilterBleDevice, FilterType};

use crate::runtime;
use crate::utils::ptr_to_string;

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
pub unsafe extern "C" fn searching_devices(ble: *mut *const BleCore, port: i64, seconds: u64) -> i64 {
    let ble_core = BleCoreSend(ble);
    let rt = runtime!();
    rt.spawn(async move {
        let ble_core = ble_core;
        let mut instance = ble_core.0.read().read();
        println!("secs : {s}", s = seconds);
        instance.scan_for_devices(Some(seconds));
        let list = instance.get_list_peripherals();
        instance.set_cache_peripherals(list);
        ble_core.0.write(&instance);
        println!("searching was finished succefully");
        Isolate::new(port).post(1);
    });
    return 1;
}


#[no_mangle]
pub unsafe extern "C" fn get_list_devices(ble: *mut *const BleCore, port: i64) {
    let ble_core = BleCoreSend(ble);
    let rt = runtime!();
    rt.spawn(async move {
        let ble_core = ble_core;
        let mut instance = ble_core.0.read().read();
        let result = match instance.get_adapter().is_none() {
            false => {
                println!("get list");
                if instance.get_cache_peripherals().is_empty() {
                    block_on(async {
                        Isolate::new(port).task(async {
                            "{\"err\":\"no peripherals was found,please start search before fetch\"}"
                        }).await;
                    });
                }
                let devices = instance.list_devices(None);
                Some(devices)
            }
            _ => {
                println!("no adapter was selected");
                block_on(async {
                    Isolate::new(port).task(async {
                        "{\"err\":\"no adapter was selected\"}"
                    }).await;
                });
                None
            }
        };
        if result.is_some() {
            block_on(async {
                let devices = result.unwrap();
                let json_devices = map_device_to_json(devices);
                println!("result ready");
                Isolate::new(port).post(json_devices);
            });
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn connect_to_device(ble: *mut *const BleCore, port: i64, address: *const c_char) {
    let ble_core = BleCoreSend(ble);
    let adr = ptr_to_string(address);
    let adr_device = adr.to_string();
    println!("address receive in rust : {a}", a = adr_device);
    let rt = runtime!();
    rt.spawn(async move {
        let ble_core = ble_core;
        let mut instance = ble_core.0.read().read();
        let result = instance.connect(FilterBleDevice {
            name: FilterType::byAdr,
            value: adr_device,
        });
        match result {
            Ok(_) => {
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
        mem::forget(result);
    });
}

#[no_mangle]
pub unsafe extern "C" fn disconnect(ble: *mut *const BleCore, port: i64) {
    let ble_core = BleCoreSend(ble);
    let rt = runtime!();
    rt.spawn(async move {
        let ble_core = ble_core;
        let mut instance = ble_core.0.read().as_ref().unwrap().clone();
        let result = instance.disconnect();
        match result {
            Ok(_) => {
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