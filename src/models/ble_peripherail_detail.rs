use btleplug::api::PeripheralProperties;
use btleplug::platform::Peripheral;

use crate::models::device_info::DeviceInfo;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct DetailPeripheral {
    pub(crate) peripheral: Peripheral,
    pub(crate) peripheral_properties: PeripheralProperties,
    pub(crate) is_connected: bool,
}


unsafe impl Send for DetailPeripheral {}

unsafe impl Sync for DetailPeripheral {}

impl DetailPeripheral {
    pub fn create_connected_detail(peri: Peripheral, propertie: PeripheralProperties) -> Self {
        return DetailPeripheral {
            peripheral: peri,
            peripheral_properties: propertie,
            is_connected: true,
        };
    }
    pub fn get_status(&self) -> bool {
        return self.is_connected;
    }
    pub fn get_peripheral(&self) -> Peripheral {
        return self.peripheral.clone();
    }
    pub fn get_properties(&self) -> PeripheralProperties {
        self.peripheral_properties.clone()
    }
    pub fn to_device_info(&self) -> DeviceInfo {
        return DeviceInfo::from(self.peripheral_properties.clone(), self.is_connected);
    }
}