use ble_desktop::common::utils::*;
use ble_desktop::models::ble_core::{BleCore, BleRepo};
use crate::utils::{run_async, RUNTIME_THREAD, runtime, error};
use allo_isolate::Isolate;

struct BleCoreSend(*mut *const BleCore);

unsafe impl Send for BleCoreSend {}

#[no_mangle]
pub unsafe extern "C" fn ble_instance(
    ble: *mut *const BleCore
) {
    let ble_core_send = BleCoreSend(ble);
    async fn open() -> Result<Arc<BleCore>> {
        let instance = BleCore::create().await;
        Ok(instance)
    }
    run_async(move || {
        let ble = ble_core_send;
        match open() {
            Ok(instance) => {
                ble.0.write(instance.as_ref());
            }
            Err(e) => {
                panic!("error to intantiate ble core")
            }
        }
    })
}

pub unsafe extern "C" fn get_list_devices(port: i64, seconds: u64) {
    let ble = BleCore::get_instance().unwrap();
    let rt = runtime!();
    rt.spawn(async move {
        let devices = ble.list_devices(seconds).await;
        let isolate = Isolate::new(port);
        let json = map_device_to_json(devices);
        isolate.post(json)
    })
}