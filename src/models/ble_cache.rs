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
        let list = &self.ble_cache_peripherals;
        return list;
    }

}