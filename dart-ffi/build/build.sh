#!/bin/bash
mode=$1
echo $mode
case $(uname | tr '[:upper:]' '[:lower:]') in
linux*)
  if [ $mode == debug ]
  then
     cargo build
  elif [ $mode == release ]
  then
     cargo build --release
  else
     exit 0
  fi
  sleep 1s
  cp "./target/$mode/libble_core_dart_ffi.so" "../packages/dart_ble_desktop/dynamicLib/libble_core_dart_ffi.so"
  ;;
darwin*)
  if [ "$1" = "x64" ]; then
    rustup target add x86_64-apple-darwin
    cargo build --release --target x86_64-apple-darwin
    sleep 1s
    mv "target/x86_64-apple-darwin/release/libble_desktop_dart_ffi.dylib" "libble_desktop_macos_x64.dylib"
  else
    rustup target add aarch64-apple-darwin
    cargo build --release --target aarch64-apple-darwin
    mv "target/aarch64-apple-darwin/release/libble_desktop_dart_ffi.dylib" "libble_desktop_macos.dylib"
  fi
  ;;
*)
  cargo build --release
  mv "target/release/libble_desktop_dart_ffi.dll" "ble_desktop_windows_x64.dll"
  ;;
esac
