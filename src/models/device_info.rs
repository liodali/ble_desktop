use btleplug::api::PeripheralProperties;
use btleplug::platform::{Peripheral};

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