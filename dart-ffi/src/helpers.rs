use ble_desktop::models::ble_cache::BleCache;

pub struct BleCacheSend(pub(crate) *mut *const BleCache);

unsafe impl Send for BleCacheSend {}

unsafe impl Sync for BleCacheSend {}


pub struct UnMutableBleCacheSend(pub(crate) *const BleCache);

unsafe impl Send for UnMutableBleCacheSend {}

unsafe impl Sync for UnMutableBleCacheSend {}