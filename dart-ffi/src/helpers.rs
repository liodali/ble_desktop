use ble_desktop::models::ble_cache::BleCache;

pub struct BleCacheSend(pub(crate) *mut *const BleCache);

unsafe impl Send for BleCacheSend {}

unsafe impl Sync for BleCacheSend {}


pub struct UnMutableBleSend<T>(pub(crate) *const T);

unsafe impl <T> Send for UnMutableBleSend<T> {}

unsafe impl <T> Sync for UnMutableBleSend<T> {}