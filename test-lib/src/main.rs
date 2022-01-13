extern crate futures;


use std::{io, thread};
use std::ops::Deref;
use std::sync::Arc;

use futures::executor::block_on;
use tokio::runtime::Builder as TokioBuilder;
use tokio::runtime::Runtime;

use ble_desktop::models::ble_core::{BleCore, BleRepo};

pub fn instantiate() -> Arc<BleCore> {
    // block_on(async {
    //
    // }).unwrap()
    BleCore::create().unwrap()
}

// #[tokio::main(flavor = "multi_thread", worker_threads = 2)]
fn main() {
    let process = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .thread_name("ble_async")
        .build();
    process.as_ref().unwrap().block_on(async {
        instantiate();
        let mut ble = BleCore::get_instance().unwrap().deref().clone();
        let adapts = ble.get_adapters().unwrap();
        let mut iters_adapts = adapts.into_iter();
        println!("len {}", iters_adapts.len());
        ble.select_default_adapter();
        let devices = ble.list_devices(Some(2),None);
        devices.into_iter().map(
            |d| d.to_string()
        ).for_each(
            |e| println!("{}", e)
        );
    });
}
