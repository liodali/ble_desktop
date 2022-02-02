use std::marker::{Send, Sync};
use std::mem;
use std::ops::Deref;
use std::os::raw::c_char;
use std::sync::{Arc, RwLock};

use allo_isolate::Isolate;
use futures::executor::block_on;
use futures::future::join_all;

use ble_desktop::common::utils::*;
use ble_desktop::models::ble_cache::BleCache;
use ble_desktop::models::ble_core::{BleCore, BleRepo};
use ble_desktop::models::ble_peripherail_detail::DetailPeripheral;
use ble_desktop::models::filter_device::{FilterBleDevice, FilterType};

use crate::helpers::{BleCacheSend, UnMutableBleSend};
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
        Isolate::new(port).post(1);
    });
}

#[no_mangle]
pub unsafe extern "C" fn instance_cache(cache: *mut *const BleCache) {
    let mut ble_cache_instance = BleCacheSend(cache);
    let instance = &mut Arc::new(BleCache::create_empty());
    ble_cache_instance.0.write(instance.as_ref());
    let n_inst = ble_cache_instance.0.read().read();
    println!("cache instantiated");
}

#[no_mangle]
pub unsafe extern "C" fn searching_devices(
    ble: *mut *const BleCore,
    cache: *mut *const BleCache,
    port: i64,
    seconds: u64,
) {
    let ble_core = BleCoreSend(ble);
    let mut ble_cache = BleCacheSend(cache);
    let rt = runtime!();
    rt.block_on(async move {
        let ble_core = ble_core;
        let ble_cache = ble_cache;
        let instance = ble_core.0.read().read();
        println!("waiting for secs : {s}", s = seconds);
        instance.scan_for_devices(Some(seconds));
        let list = instance.get_list_peripherals_with_detail();
        println!("list to be cached {}", list.len());
        let mut cache_vec = Vec::from(list.clone());
        let new_cache = Arc::new(BleCache::from_data(None, cache_vec));
        //cache_instance.set_cache_peripherals(list);
        ble_cache.0.write(new_cache.as_ref());
        let success: i32 = 1;
        Isolate::new(port).post(success);
        println!("searching was finished succefully");
    });
}


#[no_mangle]
pub unsafe extern "C" fn get_list_devices(
    ble: *mut *const BleCore,
    cache: *mut *const BleCache,
    port: i64,
) {
    //let ble_core = BleCoreSend(ble);
    let ble_cache = BleCacheSend(cache);
    let ble_cache_instance = ble_cache.0.read().read();
    let list_peris = ble_cache_instance.get_cache_peripherals();
    let rt = runtime!();
    rt.block_on(async move {
        let list_peris = list_peris;
        // let ble_core = ble_core;
        // let mut instance = ble_core.0.read().read();
        println!("len cached{}", list_peris.len());
        println!("call $list_devices");
        if list_peris.is_empty() {
            Isolate::new(port).post("[]");
        } else {
            let devices = join_all(list_peris.iter().map(|d|
                async { d.clone().to_device_info() }
            )).await;
            //let devices = instance.list_devices(peris, None);
            if !devices.is_empty() {
                let json_devices = map_device_to_json(devices);
                println!("result ready");
                Isolate::new(port).post(json_devices);
            }
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn connect_to_device(
    ble: *mut *const BleCore,
    cache: *mut *const BleCache,
    port: i64,
    address: *const c_char,
) {
    let ble_core = BleCoreSend(ble);
    let ble_cache = BleCacheSend(cache);
    let adr = ptr_to_string(address);
    let adr_device = adr.to_string();
    println!("address receive in rust : {a}", a = adr_device);
    let rt = runtime!();
    rt.spawn(async move {
        let ble_core = ble_core;
        let ble_cache = ble_cache;
        let mut instance = ble_core.0.read().read();
        let mut cache_instance = ble_cache.0.read().read();
        println!("clone list peri");
        let list = cache_instance.get_cache_peripherals();
        let filter = FilterBleDevice {
            name: FilterType::by_adr,
            value: adr_device,
        };
        let peripheral_opt = list.iter().filter(|d| {
            d.get_properties().address.to_string().contains(&filter.value)
        }).nth(0);
        match peripheral_opt {
            Some(ref detail_peri) => {
                let peri = detail_peri.get_peripheral().clone();
                let result = instance.connect(peri);
                match result {
                    Ok(_) => {
                        let success: i32 = 1;
                        Isolate::new(port).post(success);
                    }
                    _ => {
                        let fail: i32 = -1;
                        Isolate::new(port).post(fail);
                    }
                }
                mem::forget(result);
            }
            _ => {
                let fail: i32 = -400;
                Isolate::new(port).post(fail);
            }
        }
        mem::forget(peripheral_opt);
    });
}

#[no_mangle]
pub unsafe extern "C" fn disconnect(
    ble: *mut *const BleCore,
    cache: *mut *const BleCache,
    port: i64,
) {
    let ble_core = BleCoreSend(ble);
    let ble_cache = BleCacheSend(cache);
    let rt = runtime!();
    rt.spawn(async move {
        let ble_core = ble_core;
        let ble_cache = ble_cache;
        let mut instance = ble_core.0.read().read();//.as_ref().unwrap().clone();
        let mut cache_instance = ble_cache.0.read().read();
        println!("clone list peri");
        let peri = cache_instance.get_device().unwrap().get_peripheral();
        let result = instance.disconnect(peri);
        match result {
            Ok(_) => {
                let list_cache = cache_instance.get_cache_peripherals().clone();
                let n_cache_instance = BleCache::from_data(None, list_cache);
                ble_cache.0.write(&n_cache_instance);
                Isolate::new(port).post({
                    let succes_code: i32 = 1;
                    succes_code
                });
            }
            _ => {
                Isolate::new(port).post({
                    let err_code: i32 = -1;
                    err_code
                });
            }
        }
    });
}