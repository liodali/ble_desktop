use btleplug::api::PeripheralProperties;
use btleplug::api::Peripheral as _;
use btleplug::platform::Peripheral;
use futures::executor::block_on;
use futures::future::join_all;

use crate::models::device_info::*;

pub async fn get_list_properties_from_peripheral(vec_peripherals: Vec<Peripheral>) -> Vec<PeripheralProperties> {
    let properties_peripherals = join_all(vec_peripherals.iter().map(
        |p| async {
            p.to_owned().properties().await.unwrap().unwrap()
        }
    )).await;
    return properties_peripherals;
}

pub fn map_peripherals_to_device_info(vec: Vec<Peripheral>) -> Vec<DeviceInfo> {
    let len = vec.len();
    let mut vec_properties = Vec::with_capacity(len);
    let vec_peripherals = Vec::from(vec);
    println!("clone list");
    let mut list_connected_state = block_on(async {
        let mut list_connected_state: Vec<bool> = Vec::with_capacity(len);
        let vec = vec_peripherals.clone();
        for (index, peri) in vec.iter().enumerate() {
            println!("get status of {i}", i = index);
            let mut peri = peri;
            let status = peri.is_connected().await.unwrap_or(false);
            list_connected_state.push(status);
        };
        list_connected_state
    });
    /*let list_connected_state = join_all(vec_peripherals.iter().map(|p|
        async {
            p.is_connected().await.unwrap()
        }
    )).await;*/
    println!("get properties");
    let properties_peripherals = block_on(async {
        let peris = vec_peripherals.to_vec();
        get_list_properties_from_peripheral(peris).await
    });
    for (index, p) in properties_peripherals.iter().enumerate() {
        let is_connected = list_connected_state.get(index).unwrap().clone();
        let propertie = p.to_owned();
        let device = DeviceInfo::from(propertie, is_connected);
        vec_properties.push(device);
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