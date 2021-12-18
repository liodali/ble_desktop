use ble_desktop::common::*;
use ble_desktop::models::*;

extern crate futures;


use btleplug::api::ScanFilter;
use btleplug::api::{Central};
use btleplug::api::Peripheral;
use btleplug::platform::{Adapter};
use std::iter::FromIterator;
use std::time::Duration;
use futures::executor::block_on;
use tokio::time;
use ble_desktop::models::ble_core::*;
use ble_desktop::models::device_info::*;
use ble_desktop::common::utils::*;

pub async fn instantiate() -> BleCore {
    block_on(BleCore::new()).unwrap()
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
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
    let devices = ble.list_devices(Some(2)).await;
    devices.into_iter().map(
        |d| d.to_string()
    ).for_each(
        |e| println!("{}", e)
    )
}
