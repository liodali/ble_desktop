extern crate futures;

use std::ops::Deref;
use std::sync::Arc;

use ble_desktop::common::utils::get_property_from_peri;
use ble_desktop::models::ble_cache::BleCache;
use ble_desktop::models::ble_core::{BleCore, BleRepo};
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
        let listDetails = ble.get_list_peripherals_with_detail();
        cache = BleCache::from_data(None, listDetails.clone());
        let peripherals = cache.get_cache_peripherals()
            .iter().map(|detail|
            detail.get_peripheral()
        ).collect();
        let devices = ble.list_devices(peripherals, None);
        devices.clone().into_iter().map(
            |d| d.to_string()
        ).for_each(
            |e| println!("{}", e)
        );
        let list = ble.get_list_peripherals();
        println!("{:?}", list);
        let property = get_property_from_peri(list.first().unwrap().clone()).await.unwrap();
        println!("{:?}", property);
        let s = String::from("3C:20:F6:EC:31:6C");
        let detailPeri = cache.get_cache_peripherals().iter().find(|d| d.to_device_info().adr.cmp(&s).is_eq()).unwrap();
        /*
        FilterBleDevice {
            name: FilterType::by_adr,
            value: "3C:20:F6:EC:31:6C".to_string(),
        }
         */
        println!("index : {:?}", i = detailPeri.clone().get_peripheral());
        ble.connect(detailPeri.clone().get_peripheral().clone());
        cache = BleCache::from_data(Some(detailPeri.clone()), listDetails.clone());
        ble.disconnect(cache.get_device().unwrap().get_peripheral());
    });
}
