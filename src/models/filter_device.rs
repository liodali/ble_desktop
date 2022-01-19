use crate::models::device_info::*;

pub enum FilterType {
     byName,
     byAdr,
}

pub struct FilterBleDevice {
    pub name: FilterType,
    pub value: String,
}


impl FilterBleDevice {
    pub fn is_same(&self, first: DeviceInfo, second: DeviceInfo) -> Result<bool,()> {
        match self.name {
            FilterType::byName => {
                return Ok(first.name == second.name);
            }
            FilterType::byAdr => {
               return  Ok(first.adr == second.adr);
            }
        }

        Ok(false)
    }
}
