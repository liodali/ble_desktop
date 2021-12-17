#![allow(clippy::new_without_default)]

#[cfg(not(target_endian = "little"))]
compile_error!("Only little endian systems are supported.");

#[cfg(not(target_pointer_width = "64"))]
compile_error!("Only 64-bit systems are supported at this time.");
pub mod models;
pub mod common;
