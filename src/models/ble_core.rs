use std::borrow::{Borrow, BorrowMut};
use std::sync::{Arc, RwLock};
use std::thread::spawn;

use async_trait::async_trait;
use btleplug::api::{Central, Peripheral, PeripheralProperties, ScanFilter};
use btleplug::api::Manager as _;
use btleplug::Error::Other;
use btleplug::platform::{Adapter, Manager};
use btleplug::platform::Peripheral as StructPeripheral;
use btleplug::Result;
use futures::{AsyncRead, FutureExt, join, try_join};
use futures::executor::block_on;
use futures::future::{join, join_all};
use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use once_cell::sync::Lazy;

use crate::common::utils::*;
use crate::models::device_info::*;
use crate::models::filter_device::{FilterBleDevice, FilterType};

static INSTANCES: Lazy<RwLock<HashMap<u32, Arc<BleCore>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

#[derive(Debug, Clone)]
pub struct BleCore {
    ble_manager: Manager,
    ble_adapter: Option<Adapter>,
    ble_device: Option<StructPeripheral>,
}

pub trait BleRepo: Send + Sync {
    fn get_adapters(&self) -> Result<Vec<Adapter>>;
    fn set_adapter(&mut self, adapt: &Adapter);
    fn list_devices(&mut self, secs: Option<u64>, filter: Option<FilterBleDevice>) -> Vec<DeviceInfo>;
    fn start_scan(&mut self, filter: Option<ScanFilter>);
    fn stop_scan(&mut self);
    fn connect(&mut self, filter: FilterBleDevice) -> Result<()>;
    fn disconnect(&mut self) -> Result<()>;
    fn is_connected(&mut self, device: &DeviceInfo) -> Result<bool>;
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
            ble_device: None,
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

    async fn get_peripheral_by_filter(&mut self, peripheral_list: Option<Vec<StructPeripheral>>, filter: &FilterBleDevice) -> Option<StructPeripheral> {
        let mut vec_peripherals;
        let mut peripherals;
        match peripheral_list.is_some() {
            true => {
                peripherals = peripheral_list.unwrap();
            }
            _ => {
                let central = self.ble_adapter.as_ref().unwrap();
                peripherals = central.peripherals().await.unwrap()
            }
        }
        vec_peripherals = Vec::from_iter(peripherals.iter());

        let properties = get_list_properties_from_peripheral(vec_peripherals.clone()).await;
        for (index, p) in properties.iter().enumerate() {
            match filter.name {
                FilterType::byName => {
                    if p.local_name.as_ref().unwrap().contains(&filter.value)
                    {
                        let peripherals = vec_peripherals;
                        let peri = peripherals.get(index).unwrap().clone().clone();
                        return Some(peri);
                    }
                }
                FilterType::byAdr => {
                    if p.address.to_string().contains(&filter.value)
                    {
                        let peripherals = vec_peripherals;
                        let peri = peripherals.get(index).unwrap().clone().clone();
                        return Some(peri);
                    }
                }
            }
        }
        None
    }
    async fn get_peripherals_by_filter(&mut self, peripheral_list: Option<Vec<StructPeripheral>>, filter: &FilterBleDevice) -> Option<Vec<StructPeripheral>> {
        let mut vec_peripherals;
        let mut peripherals;
        match peripheral_list.is_some() {
            true => {
                peripherals = peripheral_list.unwrap();
            }
            _ => {
                let central = self.ble_adapter.as_ref().unwrap();
                peripherals = central.peripherals().await.unwrap()
            }
        }
        vec_peripherals = Vec::from_iter(peripherals.iter());

        let properties = get_list_properties_from_peripheral(vec_peripherals.clone()).await;
        let mut list = Vec::new();
        for (index, p) in properties.iter().enumerate() {
            match filter.name {
                FilterType::byName => {
                    if p.local_name.as_ref().unwrap().contains(&filter.value)
                    {
                        let peripherals = &vec_peripherals;
                        let peri = peripherals.get(index).unwrap().clone().clone();
                        list.push(peri)
                    }
                }
                FilterType::byAdr => {
                    if p.address.to_string().contains(&filter.value)
                    {
                        let peripherals = &vec_peripherals;
                        let peri = peripherals.get(index).unwrap().clone().clone();
                        list.push(peri)
                    }
                }
            }
        }
        Some(list)
    }

    async fn find_peripherals(&mut self, adapter: &Adapter, filter: Option<FilterBleDevice>) -> Vec<DeviceInfo> {
        let mut central = adapter.clone();
        let result_peripherals = central.peripherals().await;
        match result_peripherals {
            Ok(vec_peripherals) => {
                if filter.is_none() {
                    return transform_peripherals_to_properties(vec_peripherals).await.unwrap();
                }
                let opt_filter = filter.unwrap();
                let mut peripherals = self.get_peripherals_by_filter(Some(vec_peripherals), &opt_filter).await.unwrap();

                map_peripherals_to_device_info(Vec::from_iter(peripherals.iter())).await
            }
            _ => {
                panic!("error to get peripherals")
            }
        }
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
    fn list_devices(&mut self, secs: Option<u64>, filter: Option<FilterBleDevice>) -> Vec<DeviceInfo>
    {
        let adapt_option = self.get_adapter().clone();
        if adapt_option.is_none() {
            panic!("no adapter was available,please check you device")
        }
        self.start_scan(Some(ScanFilter::default()));
        let sec = if secs.is_none() { 2 } else { secs.unwrap() };
        block_on(async move {
            let sec = sec;
            std::thread::sleep(std::time::Duration::from_secs(sec));
            //sleep_fn(sec)
        });
        self.stop_scan();

        block_on(self.find_peripherals(&(adapt_option.unwrap().clone()), None))
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

    fn connect(&mut self, filter: FilterBleDevice) -> Result<()> {
        block_on(async {
            let peripheral = self.get_peripheral_by_filter(None, &filter).await.unwrap();
            let res = peripheral.connect().await;
            match res {
                Ok(()) => {
                    self.ble_device = Some(peripheral);
                    Ok(())
                }
                _ => {
                    panic!("error")
                }
            }
        })
    }

    fn disconnect(&mut self) -> Result<()> {
        block_on(async {
            let peripheral = self.ble_device.as_ref().unwrap().clone();
            self.ble_device = None;
            return peripheral.disconnect().await;
        })
    }

    fn is_connected(&mut self, device: &DeviceInfo) -> Result<bool> {
        let is_connected = block_on(async {
            let result = self.get_peripheral_by_filter(None, &FilterBleDevice {
                name: FilterType::byAdr,
                value: device.adr.clone(),
            }).await.unwrap().is_connected().await;
            match result {
                Ok(connected) => {
                    connected
                }
                _ => {
                    false
                }
            }
        });
        Ok(is_connected)
    }
}
