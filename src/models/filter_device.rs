use crate::models::device_info::*;

pub enum FilterType {
    by_name,
    by_adr,
    by_status,
}

pub struct FilterBleDevice {
    pub name: FilterType,
    pub value: String,
}


impl FilterBleDevice {
    pub fn is_same(&self, first: DeviceInfo, second: DeviceInfo) -> Result<bool, ()> {
        match self.name {
            FilterType::by_name => {
                return Ok(first.name == second.name);
            }
            FilterType::by_adr => {
                return Ok(first.adr == second.adr);
            }
            _ => {
                return Ok(first.adr == second.adr && first.name == second.name);
            }
        };

        Ok(false)
    }
}
