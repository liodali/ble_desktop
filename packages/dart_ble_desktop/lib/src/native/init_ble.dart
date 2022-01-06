import 'dart:async';
import 'dart:ffi' as ffi;

import '../common/isolate_helper.dart';
import 'ble_ffi.dart';
import 'package:ffi/ffi.dart';

import '../bluetooth_core.dart';

Future initBluetoothDesktop(String pathLib) async {
  BluetoothCore.init(pathLib);
  BleFFI.instance.storeDartPostCobject(ffi.NativeApi.postCObject);
  await _initInstanceNative();
}

Future _initInstanceNative() async {
  final bleCorePtrPtr = malloc<ffi.Pointer<ffi.NativeType>>();
  final completer = Completer<int>();
  final sendPort = singleCompletePort(completer);
  BleFFI.instance.createBleInstance(bleCorePtrPtr, sendPort.nativePort);
  final result = await completer.future;
  print("res : $result");
  //malloc.free(bleCorePtrPtr);
}
