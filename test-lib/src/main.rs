extern crate futures;


use std::ops::Deref;
use std::sync::Arc;
use futures::executor::block_on;
use ble_desktop::models::ble_core::{BleCore,BleRepo};

pub async fn instantiate() -> Arc<BleCore> {
    block_on(BleCore::create()).unwrap()
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    instantiate().await;
    let mut ble = BleCore::get_instance().unwrap().deref().clone();
    let adapts = ble.get_adapters().await.unwrap();
    let mut iters_adapts = adapts.into_iter();
    println!("{}", iters_adapts.len());
    match iters_adapts.len() == 1 {
        true => {
            let adapt = iters_adapts.nth(0).unwrap();
            ble.set_adapter(&adapt);
        }
        false => {
            println!("ble adapter available");
            // adapts.iter().map(
            //     |a| a.adapter_info()
            // ).await.for_each(
            //     |info| println!("adapt {}", info)
            // );
            println!("adapter {} selected", "");
            ble.set_adapter(&(iters_adapts.nth(0).unwrap()));
        }
    }
    let devices = ble.list_devices(Some(2)).await;
    devices.into_iter().map(
        |d| d.to_string()
    ).for_each(
        |e| println!("{}", e)
    )
}
