mod common;

use crate::common::models::*;
use crate::common::utils::*;
use btleplug::api::Manager as _;
use btleplug::api::ScanFilter;
use btleplug::platform::Manager;
use btleplug::api::{Central};
use btleplug::api::PeripheralProperties;
use btleplug::api::Peripheral;
use btleplug::platform::{Adapter};
use std::error::Error;
use std::thread;
use std::time::Duration;
use tokio::time;
use crate::common::models::DeviceInfo;


static mut MANAGER: Option<Manager> = None;
static mut DEFAULT_ADAPTER: Option<Adapter> = None;

async fn find_peripherals(central: &Adapter, filter: Option<&str>) -> Vec<PeripheralProperties> {
    let mut peripherals = Vec::new();
    if filter.is_none() | filter.unwrap().is_empty() {
        let peris = central.peripherals().await.unwrap();
        return peris.into_iter().map(
            |p| p.properties()
        ).await.into();
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
    peripherals.into_iter().map(
        |p| p.properties()
    ).await.into()
}

async unsafe fn list_devices(adapt: Option<Adapter>, secs: Option<u64>) -> Vec<DeviceInfo> {
    let my_adapt = if adapt.is_none() {
        select_adapter(0);
        DEFAULT_ADAPTER.clone()
    } else { adapt }.unwrap();
    my_adapt.start_scan(ScanFilter::default());
    let sleep = if secs.is_none() { 2 } else { secs.unwrap() };
    time::sleep(Duration::from_secs(sleep)).await;

    // find the device we're interested in
    let peripherals: Vec<PeripheralProperties> = find_peripherals(&my_adapt, None).await;
    my_adapt.stop_scan().await;
    peripherals.into_iter().map(
        |p| DeviceInfo::from(&p)
    ).await.into()
}

pub async unsafe fn list_adapter(manager: &Manager) -> Vec<Adapter> {
    let adapters = manager.adapters().await;
    adapters.unwrap()
}

pub async unsafe fn select_adapter(index: usize) {
    let adapts = list_adapter(&(MANAGER.unwrap())).await;
    let len = adapts.into_iter().len();
    match index >= len {
        true => {
            panic!(format_args!("error outIndexBounding,we have only {len} adapter", len = len))
        }
        false => {
            (&DEFAULT_ADAPTER).get_or_insert(adapts.into_iter().nth(index).unwrap());
        }
    }
}

pub async unsafe fn instantiate() -> *mut Manager {
    MANAGER.get_or_insert(Manager::new().await.unwrap())
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    unsafe {
        instantiate();
        let adapts = list_adapter(&(MANAGER.unwrap())).await;
        let len = adapts.into_iter().len();
        match len == 1 {
            true => {
                select_adapter(0).await
            }
            false => {
                println!("ble adapter available");
                adapts.into_iter().map(
                    |a| a.adapter_info()
                ).await.for_each(
                    |info| println!("adapt {}", info)
                );
                println!("adapter {} selected", "");
                select_adapter(0).await
            }
        }
        let devices = list_devices(None, Some(2)).await;
        devices.into_iter().map(
            |d| d.to_string()
        ).for_each(
            |e| println!("{}", e)
        )
    }
}
