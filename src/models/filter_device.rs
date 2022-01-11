use crate::models::device_info::*;

enum FilterType {
    byName,
    byAdr,
}

pub struct FilterBleDevice {
    pub name: FilterType,
    pub value: String,
}


impl FilterBleDevice {
    pub fn isSame(&self, first: DeviceInfo, second: DeviceInfo) -> Result<bool> {
        match self.name {
            FilterType::byName => {
                Ok(first.name == second.name)
            }
            FilterType::byAdr => {
                Ok(first.adr == second.adr)
            }
        }

        Ok(false)
    }
}
