use std::mem;
use std::ops::Deref;
use std::sync::{Arc, RwLock};

use btleplug::api::{Central, CentralEvent, Peripheral, PeripheralProperties, ScanFilter};
use btleplug::api::Manager as _;
use btleplug::platform::{Adapter, Manager};
use btleplug::platform::Peripheral as StructPeripheral;
use btleplug::Result;
use futures::{StreamExt, TryStreamExt};
use futures::executor::block_on;
use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use once_cell::sync::Lazy;

use crate::common::utils::*;
use crate::models::ble_peripherail_detail::DetailPeripheral;
use crate::models::device_info::*;
use crate::models::filter_device::{FilterBleDevice, FilterType};

static INSTANCES: Lazy<RwLock<HashMap<u32, Arc<BleCore>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BleCore {
    ble_manager: Manager,
    ble_adapter: Option<Adapter>,
    //ble_cache: BleCache,
}

// #[repr(C)]
// #[derive(Debug, Clone)]
// pub struct BleCache {
//     ble_device: Option<StructPeripheral>,
//     ble_list_peripherals: *mut Vec<StructPeripheral>,
// }

pub trait BleRepo: Send + Sync {
    fn get_adapters(&self) -> Result<Vec<Adapter>>;
    fn set_adapter(&mut self, adapt: &Adapter);
    fn scan_for_devices(&self, secs: Option<u64>);
    fn get_list_peripherals(&self) -> Vec<StructPeripheral>;
    fn get_list_peripherals_with_detail(&self) -> Vec<DetailPeripheral>;
    // fn get_cache_peripherals(&mut self) -> Vec<StructPeripheral>;
    // fn set_cache_peripherals(&mut self, vec_peripherals: Vec<StructPeripheral>);
    fn list_devices(&self, vec: Vec<StructPeripheral>, filter: Option<FilterBleDevice>) -> Vec<DeviceInfo>;
    //fn start_scan(&mut self, filter: Option<ScanFilter>);
    //fn stop_scan(&mut self);
    fn connect(&self, peripheral: StructPeripheral) -> Result<bool>;
    fn disconnect(&self, peripheral: StructPeripheral) -> Result<bool>;
    fn is_connected(&mut self, peripherals: Vec<StructPeripheral>, device: &DeviceInfo) -> Result<bool>;
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
        // let cache = BleCache {
        //     ble_device: None,
        //     ble_list_peripherals: Box::into_raw(Box::new(Vec::new())),
        // };
        Ok(Self {
            ble_manager: manager,
            ble_adapter: None,
            //ble_cache: cache,
        })
    }
    fn new_with_default_adapter() -> Result<Self> {
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
        let properties = get_list_properties_from_peripheral(vec_peripherals.clone()).await;
        for (index, p) in properties.iter().enumerate() {
            match filter.name {
                FilterType::by_name => {
                    if p.local_name.as_ref().unwrap().contains(&filter.value)
                    {
                        let peripherals = vec_peripherals;
                        let peri = peripherals.get(index).unwrap().clone().clone();
                        return Some(peri);
                    }
                }
                FilterType::by_adr => {
                    if p.address.to_string().contains(&filter.value)
                    {
                        let peripherals = vec_peripherals;
                        let peri = peripherals.get(index).unwrap().clone().clone();
                        return Some(peri);
                    }
                }
                FilterType::by_status => {
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
    fn get_peripherals_by_filter(&self, peripheral_list: Vec<StructPeripheral>, filter: &FilterBleDevice) -> Option<Vec<StructPeripheral>> {
        let peripherals = peripheral_list.clone();

        let mut vec_peripherals = Vec::from(peripherals);

        let properties = block_on(async {
            let list = vec_peripherals.clone();
            get_list_properties_from_peripheral(list).await
        });
        let mut list = Vec::new();
        for (index, p) in properties.iter().enumerate() {
            match filter.name {
                FilterType::by_name => {
                    if p.local_name.as_ref().unwrap().contains(&filter.value)
                    {
                        let peripherals = &vec_peripherals;
                        let peri = peripherals.get(index).unwrap().clone().clone();
                        list.push(peri);
                    }
                }
                FilterType::by_adr => {
                    if p.address.to_string().contains(&filter.value)
                    {
                        let peripherals = &vec_peripherals;
                        let peri = peripherals.get(index).unwrap().clone().clone();
                        list.push(peri);
                    }
                }
                FilterType::by_status => {
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

    fn find_peripherals(&self, vec: Vec<StructPeripheral>, filter: Option<FilterBleDevice>) -> Vec<DeviceInfo> {
        let peripherals = Vec::from(vec);
        if filter.is_none() {
            println!("list without filter ");
            return map_peripherals_to_device_info(peripherals);
        }
        let opt_filter = filter.unwrap();
        let filtred_peripherals = self.get_peripherals_by_filter(peripherals, &opt_filter).unwrap();

        map_peripherals_to_device_info(Vec::from(filtred_peripherals))
    }
    pub async fn start_scan(&self, filter_option: Option<ScanFilter>) {
        let filter = match filter_option {
            Some(filter) => { filter }
            _ => { ScanFilter::default() }
        };
        let adapt = self.get_adapter().unwrap().clone();
        let _r = adapt.start_scan(filter).await.expect("error to start scan");
        println!("finish scan");
    }
    pub async fn stop_scan(&self) {
        let _r = self.get_adapter().unwrap().clone().stop_scan().await;
    }
    pub async fn get_adapters_async(&self) -> Result<Vec<Adapter>> {
        self.ble_manager.adapters().await
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


    fn scan_for_devices(&self, secs: Option<u64>) {
        let sec = if secs.is_none() { 2 } else { secs.unwrap() };
        block_on(async move {
            let sec = &sec;
            println!("start scan");
            self.start_scan(Some(ScanFilter::default())).await;
            println!("sleep for seconds");
            std::thread::sleep(std::time::Duration::from_secs(*sec));
            //sleep_fn(sec)
            self.stop_scan().await;
            println!("stop scan");
        });
    }

    fn get_list_peripherals(&self) -> Vec<StructPeripheral> {
        return block_on(async move {
            //let mut central = &(adapt_option.clone());
            //adapt_option.start_scan(ScanFilter::default()).await;
            println!("call $get_adapter in $get_list_peripherals");
            let adapt = self.get_adapter().unwrap().clone();
            println!("adapter :{:?}", adapt);
            println!("finish call $get_adapter in $get_list_peripherals");
            let result_peripherals = adapt.peripherals().await.unwrap();
            println!("find peris len {}", result_peripherals.len());
            return result_peripherals;
        });
    }
    fn get_list_peripherals_with_detail(&self) -> Vec<DetailPeripheral>
    {
        let adapt = self.get_adapter().unwrap().clone();

        return block_on(async move {
            let mut detail_peris = Vec::new();
            let peripherals = adapt.peripherals().await.unwrap();
            for peri in peripherals {
                let properties = peri.properties().await.unwrap().unwrap();
                let status = peri.is_connected().await.unwrap();
                detail_peris.push(DetailPeripheral {
                    peripheral: peri,
                    peripheral_properties: properties,
                    is_connected: status,
                })
            }
            println!("{:?}", detail_peris.first().unwrap().peripheral_properties);

            return detail_peris;
        });
    }

    fn list_devices(&self, vec: Vec<StructPeripheral>, filter: Option<FilterBleDevice>) -> Vec<DeviceInfo>
    {
        if vec.clone().is_empty() {
            return Vec::new();
        }
        println!("map peris to devices");
        self.find_peripherals(vec, filter)
    }

    fn connect(&self, peripheral: StructPeripheral) -> Result<bool> {
        let result = block_on(async {
            let peripheral = peripheral;
            let res = peripheral.connect().await;
            match res {
                Ok(()) => {
                    println!("connect succefully");
                    //let _r = self.ble_cache.ble_device.insert(peripheral);
                    Ok(true)
                }
                _ => {
                    println!("error");
                    Ok(false)
                }
            }
        });
        result
    }

    fn disconnect(&self, peripheral: StructPeripheral) -> Result<bool> {
        block_on(async {
            let _e = peripheral.disconnect().await;
            return Ok(true);
        })
    }

    fn is_connected(&mut self, peripherals: Vec<StructPeripheral>, device: &DeviceInfo) -> Result<bool> {
        let is_connected = block_on(async {
            let result_peripherals = peripherals;//self.get_cache_peripherals();
            let filter = FilterBleDevice {
                name: FilterType::by_adr,
                value: device.adr.clone(),
            };
            let peri = self.get_peripheral_by_filter(
                result_peripherals,
                &filter,
            ).await.unwrap();
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
