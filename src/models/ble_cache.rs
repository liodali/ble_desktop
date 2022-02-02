use std::ops::Deref;
use std::sync::{Arc, RwLock};

use btleplug::platform::Peripheral as StructPeri;

use crate::models::ble_peripherail_detail::DetailPeripheral;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BleCache {
    ble_device_connected: Option<DetailPeripheral>,
    ble_cache_peripherals: Vec<DetailPeripheral>,
}

unsafe impl Send for BleCache {}

unsafe impl Sync for BleCache {}


impl BleCache {
    pub fn create_empty() -> Self {
        return Self {
            ble_device_connected: None,
            ble_cache_peripherals: Vec::new(),
        };
    }
    pub fn from_data(device: Option<DetailPeripheral>, list: Vec<DetailPeripheral>) -> Self {
        return Self {
            ble_device_connected: device,
            ble_cache_peripherals: Vec::from(list),
        };
    }


    pub fn get_device(&self) -> Option<DetailPeripheral> {
        self.ble_device_connected.clone()
    }

    pub fn get_cache_peripherals(&self) -> &Vec<DetailPeripheral> {
        println!("clone cache from $get_cache_peripherals");
        let list = &self.ble_cache_peripherals;
        println!("get cache list from $get_cache_peripherals");
        return list;
    }


    // pub fn set_cache_peripherals(&mut self, vec_peripherals: Vec<StructPeri>) {
    //     let mut list = &mut vec_peripherals.clone();
    //     println!("clone attr");
    //     let mut vec = unsafe {
    //         &mut *self.ble_cache_peripherals
    //     };
    //
    //     if vec.is_empty() {
    //         vec.clear();
    //         println!("cleared");
    //     }
    //     println!("copy");
    //     vec = list;
    //     println!("list len :{}", vec.len());
    //     //vec.into_iter().chain(|| { list.into_iter() });
    //     // for item in list {
    //     //     vec.push(item);
    //     // }
    //     //vec.splice(0.., list.iter().cloned());
    //     //vec = list;
    //     println!("extend");
    // }
}