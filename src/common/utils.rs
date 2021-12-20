extern crate futures;


use std::iter::FromIterator;
use btleplug::api::{Central};
use btleplug::api::Peripheral as _;
use btleplug::Error;
use btleplug::platform::Peripheral;
use btleplug::platform::Adapter;
use crate::models::device_info::*;
use futures::executor::block_on;

pub async fn transform_peripherals_to_properties(adapter: &Adapter) -> Result<Vec<DeviceInfo>, Error> {
    let peripherals = adapter.peripherals().await.unwrap();

    let vec_peripherals = Vec::from_iter(peripherals.iter());
    let properties = map_peripherals_to_properties(vec_peripherals).await;
    return Ok(properties);
}

pub async fn map_peripherals_to_properties(vec_peripherals: Vec<&Peripheral>) -> Vec<DeviceInfo> {
    let mut vec_properties = Vec::new();
    for p in vec_peripherals.iter() {
        block_on(async {
            let peri = p.to_owned().to_owned().properties().await.unwrap().unwrap();
            vec_properties.push(DeviceInfo::from(peri))
        });
    }

    return vec_properties;
}

pub fn map_device_to_json(devices: Vec<DeviceInfo>) -> String {
    let vec_json = serde_json::to_string(&devices)?;
    vec_json
}