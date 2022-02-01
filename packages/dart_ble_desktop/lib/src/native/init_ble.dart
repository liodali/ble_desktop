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
  //await _setDefaultAdapter();
}

Future _initInstanceNative() async {
  final bleCorePtrPtr = malloc<ffi.Pointer<ffi.NativeType>>();
  final bleCachePtrPtr = malloc<ffi.Pointer<ffi.NativeType>>();
  final bleFfi = BleFFI.instance;
  bleFfi.setBlePointer(bleCorePtrPtr);
  bleFfi.setBleCachePointer(bleCachePtrPtr);
  final completer = Completer<int>();
  final sendPort = singleCompletePort(completer);
  bleFfi.createBleInstance(bleCorePtrPtr, sendPort.nativePort);
  bleFfi.instantiateBleCache(bleCachePtrPtr);
  final result = await completer.future;
  print("res init instance: $result");
  //malloc.free(bleCorePtrPtr);
}
/*
Future _setDefaultAdapter() async {
  final ptr = BleFFI.instance.blePointer;
  final completer = Completer<int>();
  final sendPort = singleCompletePort(completer);
  BleFFI.instance.setDefaultAdapter(ptr, sendPort.nativePort);
  final result = await completer.future;
  print("res set adapter: $result");
  //malloc.free(bleCorePtrPtr);
}
*/
