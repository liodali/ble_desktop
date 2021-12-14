use btleplug::api::{Peripheral, PeripheralProperties};

#[derive(Debug)]
pub struct DeviceInfo {
    name: String,
    adr: String,
}


impl DeviceInfo {
    pub fn new(name: String, adr: String) -> DeviceInfo {
        DeviceInfo {
            name,
            adr,
        }
    }
    pub async fn from( p:  &PeripheralProperties) -> DeviceInfo {
        let property = p.properties().await.unwrap().unwrap();
        DeviceInfo::new(
            property.local_name.unwrap(),
            property.address.to_string(),
        )
    }
    pub fn set_name(&mut self, n: String) {
        self.name = n
    }
    pub fn set_adr(&mut self, address: String) {
        self.adr = address
    }
    pub fn to_string(&self) -> String {
        String::from(format_args!("name : {name},address : {adr}", name = self.name, adr = self.adr))
    }
}