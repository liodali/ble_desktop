import 'dart:ffi' as ffi;
import 'dart:io';

import 'package:ffi/ffi.dart';

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
      _dylib = ffi.DynamicLibrary.open("$pathLib/butplug.so");
    }
    _lookup = _dylib.lookup;
  }

  static late BleFFI instance;

  static void init(String pathLib) {
    instance = BleFFI._(pathLib);
  }

  Future createBleInstance(
    ffi.Pointer<ffi.Pointer<ffi.NativeType>> ble_ptr,
  ) async {
    _ble_create_instance(ble_ptr);
  }

  late final _ble_create_instance_ptr =
      _lookup<ffi.NativeFunction<ble_instance>>('ble_instance');
  late final _dart_ble_create_instance _ble_create_instance =
      _ble_create_instance_ptr.asFunction<_dart_ble_create_instance>();
}

typedef ble_instance = ffi.Void Function(
  ffi.Pointer<ffi.Pointer<ffi.NativeType>> ble,
);

typedef _dart_ble_create_instance = void Function(
  ffi.Pointer<ffi.Pointer<ffi.NativeType>> ble,
);
