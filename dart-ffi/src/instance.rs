use ble_desktop::common::*;
use ble_desktop::models::ble_core::BleCore;
use crate::utils::*;

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