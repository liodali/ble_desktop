use std::ops::Deref;
use std::sync::{Arc, RwLock};

use btleplug::api::{Central, Peripheral, PeripheralProperties, ScanFilter};
use btleplug::api::Manager as _;
use btleplug::platform::{Adapter, Manager};
use btleplug::platform::Peripheral as StructPeripheral;
use btleplug::Result;
use futures::executor::block_on;
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
    ble_cache: BleCache,
}

#[derive(Debug, Clone)]
pub struct BleCache {
    ble_device: Option<StructPeripheral>,
    ble_list_peripherals: *mut Vec<StructPeripheral>,
}

pub trait BleRepo: Send + Sync {
    fn get_adapters(&self) -> Result<Vec<Adapter>>;
    fn set_adapter(&mut self, adapt: &Adapter);
    fn scan_for_devices(&mut self, secs: Option<u64>);
    fn get_list_peripherals(&mut self) -> Vec<StructPeripheral>;
    fn get_cache_peripherals(&mut self) -> Vec<StructPeripheral>;
    fn set_cache_peripherals(&mut self, vec_peripherals: Vec<StructPeripheral>);
    fn list_devices(&mut self, filter: Option<FilterBleDevice>) -> Vec<DeviceInfo>;
    //fn start_scan(&mut self, filter: Option<ScanFilter>);
    //fn stop_scan(&mut self);
    fn connect(&mut self, peripheral: StructPeripheral) -> Result<bool>;
    fn disconnect(&mut self) -> Result<bool>;
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
                let instance = BleCore::new_with_default_adapter().unwrap();
                let instance_ref = e.insert(Arc::new(instance));
                Ok(instance_ref.clone())
            }
        }
    }
    pub fn create_without_adapter() -> Result<Arc<Self>> {
        let mut lock = INSTANCES.write().unwrap();
        match lock.entry(0) {
            Entry::Occupied(e) => Ok(e.get().clone()),
            Entry::Vacant(e) => {
                let instance = BleCore::new_without_adapter().unwrap();
                let instance_ref = e.insert(Arc::new(instance));
                Ok(instance_ref.clone())
            }
        }
    }
    fn new_without_adapter() -> Result<Self> {
        let manager = block_on(async {
            Manager::new().await.unwrap()
        });
        let cache = BleCache {
            ble_device: None,
            ble_list_peripherals: Box::into_raw(Box::new(Vec::new())),
        };
        Ok(Self {
            ble_manager: manager,
            ble_adapter: None,
            ble_cache: cache,
        })
    }
    fn new_with_default_adapter() -> Result<Self> {
        let manager = block_on(async {
            Manager::new().await.unwrap()
        });
        let cache = BleCache {
            ble_device: None,
            ble_list_peripherals: Box::into_raw(Box::new(Vec::new())),
        };
        let core = &mut BleCore::new_without_adapter().unwrap();
        let setAdaptFn = move |core: &mut BleCore, adpt: Adapter| {
            core.set_adapter(&adpt);
        };

        core.select_default_adapter(Some(setAdaptFn));
        Ok(core.deref().clone())
    }
    pub fn get_instance() -> Option<Arc<Self>> {
        println!("get");
        let map = INSTANCES.read().unwrap().clone();
        let ble = map.get(&0).unwrap().clone();
        Some(ble)
    }
    pub fn select_default_adapter<F>(&mut self, after: Option<F>) -> Adapter
        where F: FnMut(&mut BleCore, Adapter)
    {
        let adapters = self.get_adapters().unwrap();
        let adapter = adapters.iter().nth(0).unwrap().clone();
        match after {
            Some(mut func) => {
                func(self, adapter.clone())
            }
            _ => {}
        }
        return adapter;
        //self.set_adapter(adapter)
    }
    pub fn get_manager(&self) -> Manager {
        self.ble_manager.clone()
    }
    pub fn get_adapter(&self) -> Option<Adapter> {
        self.ble_adapter.clone()
    }

    pub async fn get_peripheral_by_filter(&mut self, peripheral_list: Vec<StructPeripheral>, filter: &FilterBleDevice) -> Option<StructPeripheral> {
        let mut peripherals = peripheral_list.clone();
        let mut vec_peripherals = Vec::from(peripherals);
        println!("filter peripherals by filter: {v}", v = filter.value.clone());
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
                FilterType::byStatus => {
                    let default_v = String::from("true").to_string();
                    let value = filter.value == default_v;
                    let peripherals = &vec_peripherals;
                    let peri = peripherals.get(index).unwrap().clone().clone();
                    let is_connected = peri.is_connected().await.unwrap();
                    if is_connected == value
                    {
                        let peri = &peri;
                        return Some(peri.clone());
                    }
                }
            }
        }
        None
    }
    fn get_peripherals_by_filter(&mut self, peripheral_list: Vec<StructPeripheral>, filter: &FilterBleDevice) -> Option<Vec<StructPeripheral>> {
        let peripherals = peripheral_list.clone();

        let mut vec_peripherals = Vec::from(peripherals);

        let properties = block_on(async {
            let list = vec_peripherals.clone();
            get_list_properties_from_peripheral(list).await
        });
        let mut list = Vec::new();
        for (index, p) in properties.iter().enumerate() {
            match filter.name {
                FilterType::byName => {
                    if p.local_name.as_ref().unwrap().contains(&filter.value)
                    {
                        let peripherals = &vec_peripherals;
                        let peri = peripherals.get(index).unwrap().clone().clone();
                        list.push(peri);
                    }
                }
                FilterType::byAdr => {
                    if p.address.to_string().contains(&filter.value)
                    {
                        let peripherals = &vec_peripherals;
                        let peri = peripherals.get(index).unwrap().clone().clone();
                        list.push(peri);
                    }
                }
                FilterType::byStatus => {
                    let value = filter.value == String::from("true").to_string();
                    let peripherals = vec_peripherals.clone();
                    let peri = peripherals.get(index).unwrap().clone();
                    let is_connected = block_on(async {
                        peri.is_connected().await.unwrap()
                    });
                    if is_connected == value
                    {
                        let peri = &peri;
                        list.push(peri.clone());
                    }
                }
            }
        }
        Some(list)
    }

    fn find_peripherals(&mut self, filter: Option<FilterBleDevice>) -> Vec<DeviceInfo> {
        println!("get cached peris");
        let peripherals = self.get_cache_peripherals();
        println!("list cached recupered");

        if filter.is_none() {
            println!("list without filter ");
            let properties = map_peripherals_to_device_info(peripherals.clone());
            return properties;
        }
        let opt_filter = filter.unwrap();
        let mut peripherals = self.get_peripherals_by_filter(peripherals, &opt_filter).unwrap();

        map_peripherals_to_device_info(Vec::from(peripherals))
    }
    pub async fn start_scan(&self, filter_option: Option<ScanFilter>) {
        let filter = match filter_option {
            Some(filter) => { filter }
            _ => { ScanFilter::default() }
        };
        println!("get adapter for scan");
        let _r = self.get_adapter().as_mut().unwrap().start_scan(filter).await;
        println!("finish scan");
    }
    pub async fn stop_scan(&self) {
        let _r = self.ble_adapter.as_ref().unwrap().stop_scan().await;
    }
}

impl BleRepo for BleCore {
    fn get_adapters(&self) -> Result<Vec<Adapter>> {
        block_on(async {
            self.ble_manager.adapters().await
        })
    }

    fn set_adapter(&mut self, adapt: &Adapter) {
        //self.ble_list_peripherals = None;
        match self.ble_adapter {
            None => {
                let adapt = adapt.to_owned();
                self.ble_adapter = Some(adapt);
            }
            _ => {
                self.ble_adapter.replace(adapt.to_owned());
            }
        }
    }
    fn get_cache_peripherals(&mut self) -> Vec<StructPeripheral> {
        let vec = unsafe {
            assert!(!self.ble_cache.ble_list_peripherals.is_null());

            &*self.ble_cache.ble_list_peripherals
        };
        println!("get");
        let list = vec.to_vec();
        println!("copy");
        list
        //self.ble_cache.ble_list_peripherals.unwrap().clone()
    }

    fn set_cache_peripherals(&mut self, vec_peripherals: Vec<StructPeripheral>) {
        let mut list = vec_peripherals.clone();

        let vec = unsafe {
            &mut *self.ble_cache.ble_list_peripherals
        };
        if !self.get_cache_peripherals().is_empty() {
            vec.clear();
        }
        vec.append(&mut list);
    }

    fn scan_for_devices(&mut self, secs: Option<u64>) {
        let sec = if secs.is_none() { 2 } else { secs.unwrap() };
        block_on(async move {
            let sec = &sec;
            println!("start scan");
            self.start_scan(Some(ScanFilter::default())).await;
            println!("sleep for secons");
            std::thread::sleep(std::time::Duration::from_secs(*sec));
            //sleep_fn(sec)
        });
        //self.stop_scan();
    }

    fn get_list_peripherals(&mut self) -> Vec<StructPeripheral> {
        println!("call $get_adapter in $get_list_peripherals");
        let adapt_option = self.get_adapter().unwrap();
        println!("finish call $get_adapter in $get_list_peripherals");
        return block_on(async {
            let mut central = &(adapt_option.clone());
            let result_peripherals = central.peripherals().await.unwrap();
            return result_peripherals;
        });
    }

    fn list_devices(&mut self, filter: Option<FilterBleDevice>) -> Vec<DeviceInfo>
    {
        println!("check cache");
        if self.get_cache_peripherals().is_empty() {
            return Vec::new();
        }
        self.find_peripherals(filter)
    }

    fn connect(&mut self, peripheral: StructPeripheral) -> Result<bool> {
        println!("start connection");
        /*println!("get ble device connected");
        let ble_device = self.ble_cache.ble_device.as_ref();
        println!("check if any device is connected");
        match ble_device {
            Some(_) => {
                println!("we found connected device,now disconnect ");
                //device.disconnect().await.expect("we cannot disconnect");
                self.ble_cache.ble_device = None;
            }
            _ => {}
        }*/
        let result = block_on(async {
            println!("search for peri");
            let peripheral = peripheral;
            println!("get peripheral to connect");
            let res = peripheral.connect().await;
            match res {
                Ok(()) => {
                    println!("connect succefully");
                    let _r = self.ble_cache.ble_device.insert(peripheral);
                    Ok(true)
                }
                _ => {
                    println!("error");
                    Ok(false)
                    //panic!("error to connect")
                }
            }
        });
        result
    }

    fn disconnect(&mut self) -> Result<bool> {
        block_on(async {
            let ble_device = &self.ble_cache.ble_device;
            match ble_device {
                Some(device) => {
                    let peripheral = device;
                    self.ble_cache.ble_device.as_ref().or(None);
                    let _e = peripheral.disconnect().await;
                    return Ok(true);
                }
                _ => {
                    return Ok(false);
                }
            }
        })
    }

    fn is_connected(&mut self, device: &DeviceInfo) -> Result<bool> {
        let is_connected = block_on(async {
            let result_peripherals = self.get_cache_peripherals();

            let peri = self.get_peripheral_by_filter(result_peripherals,
                                                     &FilterBleDevice {
                                                         name: FilterType::byAdr,
                                                         value: device.adr.clone(),
                                                     }).await.unwrap();
            let result = peri.is_connected().await;
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
