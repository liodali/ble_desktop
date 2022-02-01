import 'dart:ffi' as ffi;
import 'dart:io';

import 'package:ffi/ffi.dart';

import 'ffi_helper.dart';

class BleFFI {
  late ffi.DynamicLibrary _dylib;

  late ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  BleFFI._(String pathLib) {
    if (Platform.isMacOS) {
      _dylib = ffi.DynamicLibrary.open("$pathLib/butplug.dylib");
    }

    if (Platform.isWindows) {
      _dylib = ffi.DynamicLibrary.open("$pathLib/butplug.dll");
    }

    if (Platform.isLinux) {
      _dylib = ffi.DynamicLibrary.open("$pathLib/libble_core_dart_ffi.so");
    }
    _lookup = _dylib.lookup;
  }

  static late BleFFI instance;

  late ffi.Pointer<ffi.Pointer<ffi.NativeType>> blePointer;
  late ffi.Pointer<ffi.Pointer<ffi.NativeType>> bleCachePointer;

  static void init(String pathLib) {
    instance = BleFFI._(pathLib);
  }

  static void close() {
    malloc.free(instance.blePointer);
    malloc.free(instance.bleCachePointer);
  }

  void setBlePointer(ffi.Pointer<ffi.Pointer<ffi.NativeType>> blePointer) {
    this.blePointer = blePointer;
  }

  void setBleCachePointer(
      ffi.Pointer<ffi.Pointer<ffi.NativeType>> bleCachePointer) {
    this.bleCachePointer = bleCachePointer;
  }

  void createBleInstance(
    ffi.Pointer<ffi.Pointer<ffi.NativeType>> blePointer,
    int port,
  ) {
    _bleCreateInstance(blePointer, port);
  }

  void instantiateBleCache(
    ffi.Pointer<ffi.Pointer<ffi.NativeType>> bleCachePointer,
  ) {
    _bleCreateInstanceCache(bleCachePointer);
  }

  /*
  void setDefaultAdapter(
    ffi.Pointer<ffi.Pointer<ffi.NativeType>> blePointer,
    int port,
  ) {
    _bleSetDefaultAdapter(blePointer, port);
  }
  */

  void getListDevices(
    ffi.Pointer<ffi.Pointer<ffi.NativeType>> blePointer,
    ffi.Pointer<ffi.Pointer<ffi.NativeType>> bleCachePointer,
    int port,
  ) {
    _bleListDevices(blePointer, bleCachePointer, port);
  }

  void scanForDevices(
    ffi.Pointer<ffi.Pointer<ffi.NativeType>> blePointer,
    ffi.Pointer<ffi.Pointer<ffi.NativeType>> bleCachePointer,
    int port, {
    int seconds = 2,
  }) {
    _bleScanForDevices(blePointer, bleCachePointer, port, seconds);
  }

  void connectToDevice(
    ffi.Pointer<ffi.Pointer<ffi.NativeType>> blePointer,
    ffi.Pointer<ffi.Pointer<ffi.NativeType>> bleCachePointer,
    int port,
    String address,
  ) {
    final adr = address.toNativeUtf8();
    _connectToDevice(blePointer, bleCachePointer, port, adr);
  }

  void disconnect(
    ffi.Pointer<ffi.Pointer<ffi.NativeType>> blePointer,
    ffi.Pointer<ffi.Pointer<ffi.NativeType>> bleCachePointer,
    int port,
  ) {
    _bleDisconnect(blePointer, bleCachePointer, port);
  }

  /// Binding to `allo-isolate` crate
  void storeDartPostCobject(
    ffi.Pointer<
            ffi.NativeFunction<
                ffi.Int8 Function(ffi.Int64, ffi.Pointer<ffi.Dart_CObject>)>>
        ptr,
  ) {
    _store_dart_post_cobject(ptr);
  }

  late final _bleCreateInstancePTR =
      _lookup<ffi.NativeFunction<BleInstance>>('ble_instance');
  late final DartBleCreateInstance _bleCreateInstance =
      _bleCreateInstancePTR.asFunction<DartBleCreateInstance>();

  late final _bleCacheCreateInstancePTR =
      _lookup<ffi.NativeFunction<BleInstanceCache>>('instance_cache');
  late final DartBleCreateInstanceCache _bleCreateInstanceCache =
      _bleCacheCreateInstancePTR.asFunction<DartBleCreateInstanceCache>();

/*
  late final _bleSetDefaultAdapterPTR =
      _lookup<ffi.NativeFunction<BleSetDefaultAdapter>>(
          'select_default_adapter');
  late final DartBleCreateInstance _bleSetDefaultAdapter =
      _bleSetDefaultAdapterPTR.asFunction<DartBleCreateInstance>();
*/
  late final _bleScanForDevicesLookup =
      _lookup<ffi.NativeFunction<BleScanForDevices>>('searching_devices');
  late final DartBleScanForDevices _bleScanForDevices =
      _bleScanForDevicesLookup.asFunction<DartBleScanForDevices>();

  late final _bleListDevicesLookup =
      _lookup<ffi.NativeFunction<BleListDevices>>('get_list_devices');
  late final DartBleListDevices _bleListDevices =
      _bleListDevicesLookup.asFunction<DartBleListDevices>();

  late final _bleConnectToDeviceLookup =
      _lookup<ffi.NativeFunction<ConnectToDevice>>('connect_to_device');

  late final DartConnectToDevice _connectToDevice =
      _bleConnectToDeviceLookup.asFunction<DartConnectToDevice>();

  late final _bleDisconnectFromDeviceLookup =
      _lookup<ffi.NativeFunction<DisconnectFromDevice>>('disconnect');

  late final DartDisconnectFromDevice _bleDisconnect =
      _bleDisconnectFromDeviceLookup.asFunction<DartDisconnectFromDevice>();

  late final store_dart_post_cobject_Dart _store_dart_post_cobject = _dylib
      .lookupFunction<store_dart_post_cobject_C, store_dart_post_cobject_Dart>(
          'store_dart_post_cobject');
}
