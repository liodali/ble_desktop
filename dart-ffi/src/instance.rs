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
) {
    //let ble_core_send = BleCoreSend(ble);
    let rt = runtime!();
    let core = rt.block_on(async move {
        //let ble_core_send = ble_core_send;
        let instance = BleCore::create().unwrap();
        // ble_core_send.0.write(instance.as_ref());
        return instance.as_ref().clone();
    });
    *ble = Box::into_raw(Box::new(core));
    //Isolate::new(port).post(1);
}

#[no_mangle]
pub unsafe extern "C" fn instance_cache(cache: *mut *const BleCache) {
    //let mut ble_cache_instance = BleCacheSend(cache);
    //let instance = &mut Arc::new(BleCache::create_empty());
    //ble_cache_instance.0.write(instance.as_ref());
    let instance = BleCache::create_empty();
    *cache = Box::into_raw(Box::new(instance));
}

#[no_mangle]
pub unsafe extern "C" fn searching_devices(
    ble: &'static BleCore,
    cache: *mut *const BleCache,
    port: i64,
    seconds: u64,
) {
    let ble_core = ble;//BleCoreSend(ble);
    let mut ble_cache = BleCacheSend(cache);
    let rt = runtime!();
    let ble_cache_instance = rt.block_on(async move {
        let instance = ble_core.clone();
        let ble_cache = ble_cache;
        //let instance = ble_core.0.read().read();
        instance.scan_for_devices(Some(seconds));
        let list = instance.get_list_peripherals_with_detail();
        let mut cache_vec = Vec::from(list.clone());
        BleCache::from_data(None, cache_vec)
    });
    *cache = Box::into_raw(Box::new(ble_cache_instance));
    // let new_cache = &mut Arc::new();
    // //cache_instance.set_cache_peripherals(list);
    // ble_cache.0.write(new_cache.as_ref());
    let success: i32 = 1;
    Isolate::new(port).post(success);
}


#[no_mangle]
pub unsafe extern "C" fn get_list_devices(
    //ble: &'a BleCore,
    cache: &'static BleCache,
    port: i64,
) {
    //let ble_core = BleCoreSend(ble);
    let ble_cache = cache;
    let rt = runtime!();
    rt.spawn(async move {
        let ble_cache = ble_cache.clone();
        let list_peris = ble_cache.get_cache_peripherals().clone();
        if list_peris.is_empty() {
            Isolate::new(port).post("[]");
        } else {
            let devices = Vec::from_iter(list_peris.iter().map(|d|
                d.to_device_info().clone()
            ));
            mem::forget(list_peris);
            Isolate::new(port).task(async move {
                let res_json = map_device_to_json(devices.clone());
                res_json
            }).await;
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn connect_to_device(
    ble: &'static BleCore,
    cache: *mut *const BleCache,
    port: i64,
    address: *const c_char,
) {
    let ble_core = ble;
    let ble_cache = BleCacheSend(cache);
    let adr = ptr_to_string(address);
    let adr_device = adr.to_string();
    println!("address receive in rust : {a}", a = adr_device);
    let rt = runtime!();
    let result = rt.block_on(async move {
        let instance = ble_core.clone();
        let ble_cache = ble_cache;
        let cache_instance = ble_cache.0.read().read();
        let list = cache_instance.get_cache_peripherals().clone();
        let filter = FilterBleDevice {
            name: FilterType::by_adr,
            value: adr_device,
        };
        let peripheral_opt = list.iter().filter(|d| {
            d.get_properties().address.to_string().contains(&filter.value)
        }).nth(0);
        return match peripheral_opt {
            Some(ref detail_peri) => {
                let peri = detail_peri.get_peripheral().clone();
                let result = instance.connect(peri);
                return match result {
                    Ok(r) => {
                        if r {
                            let device_connected = detail_peri.clone().clone();
                            let n_ble_cache = BleCache::from_data(
                                Some(device_connected),
                                list,
                            );
                            return Ok(n_ble_cache);
                        } else {
                            Err(-1)
                        }
                    }
                    _ => {
                        return Err(-1);
                    }
                };
            }
            _ => {
                return Err(-400);
            }
        };
    });
    match result {
        Ok(ble_cache) => {
            *cache = Box::into_raw(Box::new(ble_cache));
            let success: i32 = 1;
            Isolate::new(port).post(success);
        }
        Err(value) => {
            let v: i32 = value;
            Isolate::new(port).post(v);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn disconnect(
    ble: &'static BleCore,
    cache: *mut *const BleCache,
    port: i64,
) {
    let ble_core = ble;
    let ble_cache = BleCacheSend(cache);
    let rt = runtime!();
    let result = rt.block_on(async move {
        let instance = ble_core.clone();
        let ble_cache = ble_cache;
        let cache_instance = ble_cache.0.read().read();
        let peri_opt = cache_instance.get_device();
        match peri_opt {
            Some(peri) => {
                let name_device = cache_instance.get_device().unwrap().to_device_info().name;
                println!("disconnection from : {}", name_device);
                let result = instance.disconnect(peri.get_peripheral());
                match result {
                    Ok(res) => {
                        match res {
                            true => {
                                Ok(cache_instance)
                            }
                            false => {
                                Err(-1)
                            }
                        }
                    }
                    _ => {
                        Err(-400)
                    }
                }
            }
            _ => {
                Err(-404)
            }
        }
    });
    match result {
        Ok(device) => {
            let device_cache = device.clone();
            let list = device_cache.get_cache_peripherals().clone();
            let n_cache_instance = BleCache::from_data(None, list);
            *cache = Box::into_raw(Box::new(n_cache_instance));
            Isolate::new(port).post({
                let succes_code: i32 = 1;
                succes_code
            });
        }
        Err(fail_err) => {
            let fail: i32 = fail_err;
            Isolate::new(port).post(fail);
        }
    }
}