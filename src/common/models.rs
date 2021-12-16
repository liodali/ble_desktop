use btleplug::api::{PeripheralProperties};
use async_trait::async_trait;
use btleplug::api::Manager as _;
use btleplug::Result;
use btleplug::platform::{Adapter, Manager};

#[derive(Debug, Clone)]
pub struct BleCore {
    /*pub(crate)*/ ble_manager: Manager,
    ble_adapter: Option<Adapter>,
}

#[async_trait]
pub trait BleRepo {
    async fn get_adapters(&self) -> Result<Vec<Adapter>>;
    fn set_adapter(&mut self, adapt: &Adapter);
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
}


#[derive(Debug, Clone)]
pub struct DeviceInfo {
    name: String,
    adr: String,
}


impl DeviceInfo {
    pub fn new(name: String, adr: String) -> Self {
        DeviceInfo {
            name,
            adr,
        }
    }
    pub fn from(p: PeripheralProperties) -> DeviceInfo {
        let property = p.clone();
        DeviceInfo::new(
            property.local_name.unwrap(),
            property.address.to_string(),
        )
    }
    #[warn(dead_code)]
    pub fn set_name(&mut self, n: String) {
        self.name = n
    }
    pub fn set_adr(&mut self, address: String) {
        self.adr = address
    }
    pub fn to_string(&self) -> String {
        String::from(format_args!("name : {name},address : {adr}", name = self.name, adr = self.adr).to_string())
    }
}