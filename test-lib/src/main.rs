extern crate futures;

use std::ops::Deref;
use std::sync::Arc;
use ble_desktop::common::utils::get_property_from_peri;

use ble_desktop::models::ble_core::{ BleCore, BleRepo};
use ble_desktop::models::ble_cache::BleCache;
use ble_desktop::models::filter_device::{FilterBleDevice, FilterType};

pub fn instantiate() -> Arc<BleCore> {
    // block_on(async {
    //
    // }).unwrap()
    BleCore::create().unwrap()
}

// #[tokio::main(flavor = "multi_thread", worker_threads = 2)]
fn main() {
    let process = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .thread_name("ble_async")
        .build();
    process.as_ref().unwrap().block_on(async {
        instantiate();
        let mut cache = BleCache::create_empty();
        let mut ble = BleCore::get_instance().unwrap().deref().clone();
        let adapts = ble.get_adapters().unwrap();
        let iters_adapts = adapts.into_iter();
        println!("len {}", iters_adapts.len());
        //ble.select_default_adapter(None);
        ble.scan_for_devices(Some(2));
        let list = ble.get_list_peripherals();
        cache = BleCache::from_data(None,list);
        let devices = ble.list_devices( cache.get_cache_peripherals(),None);
        devices.into_iter().map(
            |d| d.to_string()
        ).for_each(
            |e| println!("{}", e)
        );
        let list = ble.get_list_peripherals();
        println!("{:?}",list);
        let property = get_property_from_peri(list.first().unwrap().clone()).await.unwrap();
        println!("{:?}",property);
        // ble.connect(FilterBleDevice {
        //     name: FilterType::byAdr,
        //     value: "3C:20:F6:EC:31:6C".to_string(),
        // });
        // ble.disconnect();
    });
}
