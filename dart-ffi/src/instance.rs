use std::marker::{Send, Sync};
use std::mem;
use std::ops::Deref;
use std::os::raw::c_char;
use std::sync::{Arc, RwLock};

use allo_isolate::Isolate;
use futures::executor::block_on;

use ble_desktop::common::utils::*;
use ble_desktop::models::ble_cache::BleCache;
use ble_desktop::models::ble_core::{BleCore, BleRepo};
use ble_desktop::models::filter_device::{FilterBleDevice, FilterType};

use crate::helpers::{BleCacheSend, UnMutableBleCacheSend};
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
            1
        });
    });
}

#[no_mangle]
pub unsafe extern "C" fn instance_cache(cache: *mut *const BleCache) {
    let mut ble_cache_instance = BleCacheSend(cache);
    let instance = &mut Arc::new(BleCache::create_empty());
    ble_cache_instance.0.write(instance.as_ref());
    let n_inst = ble_cache_instance.0.read().read();
    println!("check size of  empty list cached,{}", n_inst.get_cache_peripherals().len());
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
        let mut instance = ble_core.0.read().read();
        println!("waiting for secs : {s}", s = seconds);
        instance.scan_for_devices(Some(seconds));
        let list = instance.get_list_peripherals();
        println!("list to be cached {}", list.len());
        let first = list.first().unwrap().clone();
        let is_ctx = get_status_from_peri(first).await.unwrap();
        println!("status of first elem {}", is_ctx);
        let mut cache_vec = Vec::from(list.clone());
        let new_cache = Arc::new(BleCache::from_data(None, cache_vec));
        //cache_instance.set_cache_peripherals(list);
        ble_cache.0.write(new_cache.as_ref());
        Isolate::new(port).post(1);
        println!("searching was finished succefully");
    });

}


#[no_mangle]
pub unsafe extern "C" fn get_list_devices(
    ble: *mut *const BleCore,
    cache: *mut *const BleCache,
    port: i64,
) {
    let ble_core = BleCoreSend(ble);
    let ble_cache = BleCacheSend(cache);
    let ble_cache_instance = ble_cache.0.read().read();
    let list_peris = ble_cache_instance.get_cache_peripherals();
    let rt = runtime!();
    rt.block_on(async move {
        let ble_core = ble_core;
        let mut instance = ble_core.0.read().read();
        block_on(async{
            instance.start_scan(None).await;
            std::thread::sleep(std::time::Duration::from_secs(1));
        });
        let list_peris = instance.get_list_peripherals();
        println!("len {}", list_peris.len());
        let first = list_peris.first().unwrap().clone();
        let is_ctx = block_on(async { get_status_from_peri(first.clone()).await }).unwrap();
        println!("status of first elem {}", is_ctx);
        println!("call $list_devices");
        if list_peris.is_empty() {
            Isolate::new(port).post("[]");
        } else {
            let len = list_peris.len();
            let devices = instance.list_devices(list_peris, None);
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
        let peripheral_opt = instance.get_peripheral_by_filter(
            list,
            &filter,
        ).await;
        match peripheral_opt {
            Some(ref peri) => {
                let result = instance.connect(peri.clone());
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
            }
            _ => {
                Isolate::new(port).post({
                    -400
                });
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
        let peri = cache_instance.get_device().unwrap();
        let result = instance.disconnect(peri);
        match result {
            Ok(_) => {
                let n_cache_instance = BleCache::from_data(None, cache_instance.get_cache_peripherals());
                ble_cache.0.write(&n_cache_instance);
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