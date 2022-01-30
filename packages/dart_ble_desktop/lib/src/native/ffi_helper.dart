import 'dart:ffi' as ffi;

import 'package:ffi/ffi.dart';




typedef BleInstance = ffi.Void Function(
  ffi.Pointer<ffi.Pointer<ffi.NativeType>> ble,
  ffi.Int64 port,
);

typedef DartBleCreateInstance = void Function(
  ffi.Pointer<ffi.Pointer<ffi.NativeType>> ble,
  int port,
);
/*
typedef BleSetDefaultAdapter = ffi.Void Function(
  ffi.Pointer<ffi.Pointer<ffi.NativeType>> ble,
  ffi.Int64 port,
);

typedef DartBleSetDefaultAdapter = void Function(
  ffi.Pointer<ffi.Pointer<ffi.NativeType>> ble,
  int port,
);
*/
typedef BleScanForDevices = ffi.Void Function(
  ffi.Pointer<ffi.Pointer<ffi.NativeType>> ble,
  ffi.Int64 port,
  ffi.Int64 seconds,
);

typedef DartBleScanForDevices = void Function(
  ffi.Pointer<ffi.Pointer<ffi.NativeType>> ble,
  int port,
  int seconds,
);


typedef BleListDevices = ffi.Void Function(
  ffi.Pointer<ffi.Pointer<ffi.NativeType>> ble,
  ffi.Int64 port,
);

typedef DartBleListDevices = void Function(
  ffi.Pointer<ffi.Pointer<ffi.NativeType>> ble,
  int port,
);




/// binding connectToDevice from ffi to dart
typedef ConnectToDevice = ffi.Void Function(
  ffi.Pointer<ffi.Pointer<ffi.NativeType>> ble,
  ffi.Int64 port,
  ffi.Pointer<Utf8> address,
);
typedef DartConnectToDevice = void Function(
  ffi.Pointer<ffi.Pointer<ffi.NativeType>> ble,
  int port,
  ffi.Pointer<Utf8> address,
);

/// binding disconnect method from ffi to dart
typedef DisconnectFromDevice = ffi.Void Function(
  ffi.Pointer<ffi.Pointer<ffi.NativeType>> ble,
  ffi.Int64 port,
);
typedef DartDisconnectFromDevice = void Function(
  ffi.Pointer<ffi.Pointer<ffi.NativeType>> ble,
  int port,
);


/// binding dart object to cobject 
typedef store_dart_post_cobject_C = ffi.Void Function(
  ffi.Pointer<
          ffi.NativeFunction<
              ffi.Int8 Function(ffi.Int64, ffi.Pointer<ffi.Dart_CObject>)>>
      ptr,
);
typedef store_dart_post_cobject_Dart = void Function(
  ffi.Pointer<
          ffi.NativeFunction<
              ffi.Int8 Function(ffi.Int64, ffi.Pointer<ffi.Dart_CObject>)>>
      ptr,
);
