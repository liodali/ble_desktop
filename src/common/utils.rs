use btleplug::api::Peripheral as _;
use btleplug::api::PeripheralProperties;
use btleplug::platform::Peripheral;
use futures::executor::block_on;
use futures::future::join_all;

use crate::models::device_info::*;

pub async fn get_property_from_peri(peripheral: Peripheral) -> Result<PeripheralProperties,()> {
    return Ok(peripheral.properties().await.unwrap().unwrap());
}

pub async fn get_status_from_peri(peripheral: Peripheral) -> Result<bool,()> {
    return Ok(peripheral.is_connected().await.unwrap());
}

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
    let mut list_details = block_on(async {
        let mut list_details: Vec<(bool, PeripheralProperties)> = Vec::with_capacity(len);
        let vec = vec_peripherals.clone();
        for (index, peri) in vec.iter().enumerate() {
            println!("get status of {i}", i = index);
            let mut peri = peri.clone();
            println!("get propertie");
            let propertie = peri.properties().await.unwrap().unwrap();
            println!("get statu");
            let status = peri.is_connected().await.unwrap_or(false);
            let detail = (status, propertie);
            list_details.push(detail);
        };
        list_details
    });
    /*let list_connected_state = join_all(vec_peripherals.iter().map(|p|
        async {
            p.is_connected().await.unwrap()
        }
    )).await;*/
    println!("get properties");
    // let properties_peripherals = block_on(async {
    //     let peris = vec_peripherals.to_vec();
    //     get_list_properties_from_peripheral(peris).await
    // });
    for detail in list_details {
        let is_connected = detail.0;
        let propertie = detail.1;
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