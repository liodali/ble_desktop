use btleplug::api::PeripheralProperties;
use btleplug::platform::Peripheral;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[repr(C)]
pub struct DeviceInfo {
    pub name: String,
    pub adr: String,
    pub is_connected: bool,
}


impl DeviceInfo {
    pub fn new(name: String, adr: String, is_connected: Option<bool>) -> Self {
        DeviceInfo {
            name,
            adr,
            is_connected: match is_connected {
                Some(connected) => {
                    connected
                }
                _ => {
                    false
                }
            },
        }
    }
    pub fn from(p: PeripheralProperties, is_connected: bool) -> DeviceInfo {
        let property = p.clone();
        DeviceInfo::new(
            property.local_name.unwrap(),
            property.address.to_string(),
            Some(is_connected),
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
        String::from(format_args!("name : {name},address : {adr},connected:{connect}", name = self.name, adr = self.adr, connect = self.is_connected).to_string())
    }

    pub fn to_json(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }

    pub fn compare_with(&self, other: DeviceInfo) -> bool {
        self.adr == other.adr && self.name == self.name
    }
}