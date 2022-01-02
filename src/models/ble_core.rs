use btleplug::api::{PeripheralProperties, ScanFilter, Central, Peripheral};
use async_trait::async_trait;
use btleplug::api::Manager as _;
use btleplug::Result;
use btleplug::platform::{Adapter, Manager};
use btleplug::platform::Peripheral as StructPeripheral;
use crate::models::device_info::*;
use crate::common::utils::*;
use std::{thread, time::Duration};
use hashbrown::HashMap;
use hashbrown::hash_map::Entry;
use once_cell::sync::Lazy;
use std::sync::{RwLock,Arc};

static INSTANCES: Lazy<RwLock<HashMap<u32, Arc<BleCore>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

#[derive(Debug, Clone)]
pub struct BleCore {
    ble_manager: Manager,
    ble_adapter: Option<Adapter>,
}

#[async_trait]
pub trait BleRepo: Send + Sync {
    async fn get_adapters(&self) -> Result<Vec<Adapter>>;
    fn set_adapter(&mut self, adapt: &Adapter);
    async fn list_devices(&mut self, secs: Option<u64>) -> Vec<DeviceInfo>;
    async fn start_scan(&mut self, filter: Option<ScanFilter>);
    async fn stop_scan(&mut self);
    async fn connect(&mut self, device: DeviceInfo) -> Result<()>;
    async fn disconnect(&mut self, device: DeviceInfo) -> Result<()>;
    //async fn find_peripherals(&mut self, filter: Option<&str>) -> Vec<DeviceInfo>;
}

unsafe impl Send for BleCore {}

unsafe impl Sync for BleCore {}

impl BleCore {
    pub async fn create() -> Result<Arc<Self>> {
        let mut lock = INSTANCES.write().unwrap();
        match lock.entry(0) {
            Entry::Occupied(e) => Ok(e.get().clone()),
            Entry::Vacant(e) => {
                let instance = BleCore::new().await.unwrap();
                let instance_ref = e.insert(Arc::new(instance));
                Ok(instance_ref.clone())
            }
        }
    }
    async fn new() -> Result<Self> {
        Ok(Self {
            ble_manager: Manager::new().await.unwrap(),
            ble_adapter: None,
        })
    }
    pub fn get_instance() -> Option<Arc<Self>> {
        let map = INSTANCES.read().unwrap().clone();
        map.get(&0).cloned()
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

    async fn get_peripherals_by_device(&mut self, device: &DeviceInfo) -> Option<StructPeripheral> {
        let central = self.ble_adapter.as_ref().unwrap();
        for p in central.peripherals().await.unwrap() {
            if p.properties()
                .await
                .unwrap()
                .unwrap()
                .local_name
                .iter()
                .any(|name| name.contains(&device.name))
            {
                return Some(p);
            }
        }
        None
    }

    async fn find_peripherals(&mut self, filter: Option<&str>) -> Vec<DeviceInfo> {
        let mut peripherals = Vec::new();
        let central = self.ble_adapter.as_ref().unwrap();
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
        my_adapt.start_scan(ScanFilter::default()).await;
        let sleep = if secs.is_none() { 2 } else { secs.unwrap() };
        // time::sleep(Duration::from_secs(sleep)).await;
        thread::sleep(Duration::from_secs(sleep));
        // find the device we're interested in
        let peripherals: Vec<DeviceInfo> = self.find_peripherals(None).await;
        my_adapt.stop_scan().await;
        peripherals
    }

    async fn start_scan(&mut self, filter: Option<ScanFilter>) {
        self.ble_adapter.as_ref().unwrap().start_scan(filter.unwrap());
    }

    async fn stop_scan(&mut self) {
        self.ble_adapter.as_ref().unwrap().stop_scan().await;
    }

    async fn connect(&mut self, device: DeviceInfo) -> Result<()> {
        let peripheral = self.get_peripherals_by_device(&device).await;
        return peripheral.unwrap().connect().await;
    }

    async fn disconnect(&mut self, device: DeviceInfo) -> Result<()> {
        let peripheral = self.get_peripherals_by_device(&device).await;
        return peripheral.unwrap().disconnect().await;
    }
}
