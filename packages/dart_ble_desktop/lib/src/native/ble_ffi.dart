import 'dart:ffi' as ffi;
import 'dart:io';

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

  static void init(String pathLib) {
    instance = BleFFI._(pathLib);
  }

  void createBleInstance(
    ffi.Pointer<ffi.Pointer<ffi.NativeType>> blePointer,
    int port,
  ) {
    _bleCreateInstance(blePointer,port);
  }

  void getListDevices(
    int port, {
    int seconds = 2,
  }) {
    _bleListDevices(port, seconds);
  }

  /// Binding to `allo-isolate` crate
  void storeDartPostCobject(
    ffi.Pointer<
            ffi.NativeFunction<
                ffi.Int8 Function(ffi.Int64, ffi.Pointer<ffi.Dart_CObject>)>>
        ptr,
  ) {
    final _store_dart_post_cobject_Dart _store_dart_post_cobject =
        _dylib.lookupFunction<_store_dart_post_cobject_C,
            _store_dart_post_cobject_Dart>('store_dart_post_cobject');

    _store_dart_post_cobject(ptr);
  }

  late final _bleCreateInstancePTR =
      _lookup<ffi.NativeFunction<BleInstance>>('ble_instance');
  late final DartBleCreateInstance _bleCreateInstance =
      _bleCreateInstancePTR.asFunction<DartBleCreateInstance>();

  late final _bleListDevicesLookup =
      _lookup<ffi.NativeFunction<BleListDevices>>('get_list_devices');
  late final DartBleListDevices _bleListDevices =
      _bleListDevicesLookup.asFunction<DartBleListDevices>();
}

typedef BleInstance = ffi.Void Function(
  ffi.Pointer<ffi.Pointer<ffi.NativeType>> ble,
  ffi.Int64 port,
);

typedef DartBleCreateInstance = void Function(
  ffi.Pointer<ffi.Pointer<ffi.NativeType>> ble,
  int port,
);

typedef BleListDevices = ffi.Void Function(
  ffi.Int64 port,
  ffi.Int64 seconds,
);

typedef DartBleListDevices = void Function(
  int port,
  int seconds,
);

typedef _store_dart_post_cobject_C = ffi.Void Function(
  ffi.Pointer<
          ffi.NativeFunction<
              ffi.Int8 Function(ffi.Int64, ffi.Pointer<ffi.Dart_CObject>)>>
      ptr,
);
typedef _store_dart_post_cobject_Dart = void Function(
  ffi.Pointer<
          ffi.NativeFunction<
              ffi.Int8 Function(ffi.Int64, ffi.Pointer<ffi.Dart_CObject>)>>
      ptr,
);
