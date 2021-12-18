use btleplug::api::{PeripheralProperties, ScanFilter, Central, Peripheral};
use async_trait::async_trait;
use btleplug::api::Manager as _;
use btleplug::Result;
use btleplug::platform::{Adapter, Manager};
use crate::models::device_info::*;
use crate::common::utils::*;
use tokio::time;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct BleCore {
    ble_manager: Manager,
    ble_adapter: Option<Adapter>,
}

#[async_trait]
pub trait BleRepo {
    async fn get_adapters(&self) -> Result<Vec<Adapter>>;
    fn set_adapter(&mut self, adapt: &Adapter);
    async fn list_devices(&mut self, secs: Option<u64>) -> Vec<DeviceInfo>;
    //async fn find_peripherals(&mut self, filter: Option<&str>) -> Vec<DeviceInfo>;
}

impl BleCore {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            ble_manager: Manager::new().await.unwrap(),
            ble_adapter: None,
        })
    }
    pub async fn select_default_adapter(&mut self) {
        let adapters = self.get_adapters().await.unwrap();
        let adapter = adapters.iter().nth(0).unwrap();
        self.set_adapter(adapter)
    }
    pub fn get_manager(&self) -> Manager {
        self.ble_manager.clone()
    }
    pub fn get_adapter(&self) -> Option<Adapter> {
        self.ble_adapter.clone()
    }

    async fn find_peripherals(&mut self,filter: Option<&str>) -> Vec<DeviceInfo> {
        let mut peripherals = Vec::new();
        let central = (self.ble_adapter.as_ref().unwrap());
        if filter.is_none() || filter.unwrap().is_empty() {
            return transform_peripherals_to_properties(central).await.unwrap();
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
}

#[async_trait]
impl BleRepo for BleCore {
    async fn get_adapters(&self) -> Result<Vec<Adapter>> {
        self.ble_manager.adapters().await
    }

    fn set_adapter(&mut self, adapt: &Adapter) {
        match self.ble_adapter {
            None => {
                self.ble_adapter = Some(adapt.to_owned().to_owned())
            }
            _ => {
                self.ble_adapter.replace(adapt.to_owned().to_owned());
            }
        }
    }
    async fn list_devices(&mut self, secs: Option<u64>) -> Vec<DeviceInfo> {
        if self.get_adapter().is_none() {
            panic!("no adapter was available,please check you device")
        }
        let my_adapt = self.get_adapter().unwrap();
        my_adapt.start_scan(ScanFilter::default());
        let sleep = if secs.is_none() { 2 } else { secs.unwrap() };
        time::sleep(Duration::from_secs(sleep)).await;

        // find the device we're interested in
        let peripherals: Vec<DeviceInfo> = self.find_peripherals(None).await;
        my_adapt.stop_scan().await;
        peripherals
    }

}
