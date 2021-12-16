mod common;

use crate::common::models::*;

extern crate futures;

use btleplug::api::ScanFilter;
use btleplug::api::{Central};
use btleplug::api::Peripheral;
use btleplug::platform::{Adapter};
use std::iter::FromIterator;
use std::time::Duration;
use futures::executor::block_on;
use tokio::time;
use crate::common::models::DeviceInfo;
use crate::common::utils::{map_peripherals_to_properties, transform_peripherals_to_properties};


async fn find_peripherals(central: &Adapter, filter: Option<&str>) -> Vec<DeviceInfo> {
    let mut peripherals = Vec::new();
    if filter.is_none() || filter.unwrap().is_empty() {
        return transform_peripherals_to_properties(&central).await.unwrap();
    }
    for p in central.peripherals().await.unwrap() {
        if p.properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .iter()
            .any(|name| name.contains(filter.unwrap()))
        {
            peripherals.push(p);
        }
    }

    map_peripherals_to_properties(Vec::from_iter(peripherals.iter())).await
}

async  fn list_devices(ble_core: &mut BleCore, secs: Option<u64>) -> Vec<DeviceInfo> {
    let my_adapt = ble_core.get_adapter().unwrap();
    my_adapt.start_scan(ScanFilter::default());
    let sleep = if secs.is_none() { 2 } else { secs.unwrap() };
    time::sleep(Duration::from_secs(sleep)).await;

    // find the device we're interested in
    let peripherals: Vec<DeviceInfo> = find_peripherals(&my_adapt, None).await;
    my_adapt.stop_scan().await;
    peripherals
}

pub async fn instantiate() -> BleCore {
    block_on(BleCore::new()).unwrap()
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async  fn main() {
    let mut ble = instantiate().await;
    let adapts = ble.get_adapters().await.unwrap();
    let mut iters_adapts = adapts.into_iter();
    println!("{}", iters_adapts.len());
    match iters_adapts.len() == 1 {
        true => {
            let adapt = iters_adapts.nth(0).unwrap();
            ble.set_adapter(&adapt);
        }
        false => {
            println!("ble adapter available");
            // adapts.iter().map(
            //     |a| a.adapter_info()
            // ).await.for_each(
            //     |info| println!("adapt {}", info)
            // );
            println!("adapter {} selected", "");
            ble.set_adapter(&(iters_adapts.nth(0).unwrap()));
        }
    }
    let devices = list_devices(&mut ble, Some(2)).await;
    devices.into_iter().map(
        |d| d.to_string()
    ).for_each(
        |e| println!("{}", e)
    )
}
