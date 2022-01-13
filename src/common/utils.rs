use std::iter::FromIterator;

use btleplug::api::{Central, PeripheralProperties};
use btleplug::api::Peripheral as _;
use btleplug::Error;
use btleplug::platform::Adapter;
use btleplug::platform::Peripheral;
use futures::executor::block_on;
use futures::future::join_all;

use crate::models::device_info::*;

pub async fn transform_peripherals_to_properties(adapter: &Adapter) -> Result<Vec<DeviceInfo>, Error> {
    let adapter = adapter.clone();
    let peripherals_result = adapter.peripherals().await;
    match peripherals_result {
        Ok(result) => {
            let peripherals = result;
            let vec_peripherals = Vec::from_iter(peripherals.iter());
            let properties = map_peripherals_to_device_info(vec_peripherals).await;
            return Ok(properties);
        }
        _ => {
            panic!("error to get peripherals")
        }
    };
}

pub async fn get_list_properties_from_peripheral(vec_peripherals: Vec<&Peripheral>) -> Vec<PeripheralProperties> {
    let properties_peripherals = join_all(vec_peripherals.iter().map(
        |p| async {
            p.to_owned().to_owned().properties().await.unwrap().unwrap()
        }
    )).await;
    return properties_peripherals;
}

pub async fn map_peripherals_to_device_info(vec_peripherals: Vec<&Peripheral>) -> Vec<DeviceInfo> {
    let mut vec_properties = Vec::new();
    let vec_peripherals = vec_peripherals;
    let list_connected_state = join_all(vec_peripherals.iter().map(|p|
        async {
            p.is_connected().await.unwrap()
        }
    )).await;
    let properties_peripherals = get_list_properties_from_peripheral(vec_peripherals).await;
    for (index, p) in properties_peripherals.iter().enumerate() {
        let is_connected = list_connected_state.get(index).unwrap().clone();
        let propertie = p.to_owned();
        vec_properties.push(DeviceInfo::from(propertie, is_connected))
    }
    return vec_properties;
}

pub fn map_device_to_json(devices: Vec<DeviceInfo>) -> String {
    let vec_json = serde_json::to_string(&devices);
    match vec_json {
        Ok(data_json) => {
            data_json
        }
        _ => {
            panic!("error to parse data to json")
        }
    }
}