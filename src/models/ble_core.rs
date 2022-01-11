use std::borrow::Borrow;
use std::sync::{Arc, RwLock};
use std::thread::spawn;

use async_trait::async_trait;
use btleplug::api::{Central, Peripheral, PeripheralProperties, ScanFilter};
use btleplug::api::Manager as _;
use btleplug::platform::{Adapter, Manager};
use btleplug::platform::Peripheral as StructPeripheral;
use btleplug::Result;
use futures::{AsyncRead, join, try_join};
use futures::executor::block_on;
use futures::future::{join, join_all};
use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use once_cell::sync::Lazy;

use crate::common::utils::*;
use crate::models::device_info::*;

static INSTANCES: Lazy<RwLock<HashMap<u32, Arc<BleCore>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

#[derive(Debug, Clone)]
pub struct BleCore {
    ble_manager: Manager,
    ble_adapter: Option<Adapter>,
}

pub trait BleRepo: Send + Sync {
    fn get_adapters(&self) -> Result<Vec<Adapter>>;
    fn set_adapter(&mut self, adapt: &Adapter);
    fn list_devices(&mut self, secs: Option<u64>) -> Vec<DeviceInfo>;
    fn start_scan(&mut self, filter: Option<ScanFilter>);
    fn stop_scan(&mut self);
    fn connect(&mut self, device: DeviceInfo) -> Result<()>;
    fn disconnect(&mut self, device: DeviceInfo) -> Result<()>;
    //async fn find_peripherals(&mut self, filter: Option<&str>) -> Vec<DeviceInfo>;
}

unsafe impl Send for BleCore {}

unsafe impl Sync for BleCore {}

impl BleCore {
    pub fn create() -> Result<Arc<Self>> {
        let mut lock = INSTANCES.write().unwrap();
        match lock.entry(0) {
            Entry::Occupied(e) => Ok(e.get().clone()),
            Entry::Vacant(e) => {
                let instance = BleCore::new().unwrap();
                println!("init");
                let instance_ref = e.insert(Arc::new(instance));
                Ok(instance_ref.clone())
            }
        }
    }
    fn new() -> Result<Self> {
        let manager = block_on(Manager::new()).unwrap();
        Ok(Self {
            ble_manager: manager,
            ble_adapter: None,
        })
    }
    pub fn get_instance() -> Option<Arc<Self>> {
        println!("get");
        let map = INSTANCES.read().unwrap().clone();
        let ble = map.get(&0).unwrap().clone();
        Some(ble)
    }
    pub fn select_default_adapter(&mut self) {
        let adapters = self.get_adapters().unwrap();
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

    async fn find_peripherals(&mut self, adapter: &Adapter, filter: Option<&str>) -> Vec<DeviceInfo> {
        let mut peripherals = Vec::new();
        let mut central = adapter.clone();
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
}

impl BleRepo for BleCore {
    fn get_adapters(&self) -> Result<Vec<Adapter>> {
        block_on(async {
            self.ble_manager.adapters().await
        })
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
    fn list_devices(&mut self, secs: Option<u64>,filter: Option<FilterBleDevice>) -> Vec<DeviceInfo>
    {
        let mut adapt_option = self.get_adapter().clone();
        if adapt_option.is_none() {
            panic!("no adapter was available,please check you device")
        }
        let my_adapt = adapt_option.unwrap().clone();
        self.start_scan(Some(ScanFilter::default()));
        let sec = if secs.is_none() { 2 } else { secs.unwrap() };
        block_on(async move {
            let sec = sec;
            std::thread::sleep(std::time::Duration::from_secs(sec));
            //sleep_fn(sec)
        });
        self.stop_scan();

        block_on(self.find_peripherals(&my_adapt, None))
    }

    fn start_scan(&mut self, filter: Option<ScanFilter>) {
        block_on(async {
            self.ble_adapter.as_ref().unwrap().start_scan(filter.unwrap()).await;
        })
    }

    fn stop_scan(&mut self) {
        block_on(async {
            self.ble_adapter.as_ref().unwrap().stop_scan().await;
        })
    }

    fn connect(&mut self, device: DeviceInfo) -> Result<()> {
        block_on(async {
            let peripheral = self.get_peripherals_by_device(&device).await;
            peripheral.unwrap().connect().await
        });
        Ok(())
    }

    fn disconnect(&mut self, device: DeviceInfo) -> Result<()> {
        block_on(async {
            let peripheral = self.get_peripherals_by_device(&device).await;
            return peripheral.unwrap().disconnect().await;
        });
        Ok(())
    }
}
