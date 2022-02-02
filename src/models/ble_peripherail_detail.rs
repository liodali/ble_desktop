use btleplug::api::PeripheralProperties;
use btleplug::platform::Peripheral;

use crate::models::device_info::DeviceInfo;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct DetailPeripheral {
    pub(crate) peripheral_info: (Peripheral, PeripheralProperties),
    pub(crate) is_connected: bool,
}


unsafe impl Send for DetailPeripheral {}

unsafe impl Sync for DetailPeripheral {}

impl DetailPeripheral {
    pub fn get_status(&self) -> bool {
        return self.is_connected;
    }
    pub fn get_peripheral(&self) -> Peripheral {
        return self.peripheral_info.0.clone();
    }
    pub fn get_properties(&self) -> PeripheralProperties {
        self.peripheral_info.1.clone()
    }
    pub fn to_device_info(&self) -> DeviceInfo {
        return DeviceInfo::from(self.peripheral_info.1.clone(), self.is_connected);
    }
}