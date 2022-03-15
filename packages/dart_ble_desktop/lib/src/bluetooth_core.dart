import 'dart:async';
import 'dart:convert';
import 'package:dart_ble_desktop/src/models/device.dart';
import 'dart:ffi';
import 'package:flutter/material.dart';

import 'common/isolate_helper.dart';
import 'models/exceptions.dart';
import 'native/ble_ffi.dart';

abstract class BluetoothCore {
  late BleFFI _bleFFI;

  static late final BluetoothCore? _instance;

  static BluetoothCore getInstance() {
    _instance ??= BluetoothCoreImpl.setUp();

    return _instance!;
  }

  static init(String namePathLib) {
    BleFFI.init(namePathLib);
  }

  static close() {
    BleFFI.close();
  }

  BluetoothCore._() {
    _bleFFI = BleFFI.instance;
  }

  @mustCallSuper
  dispose() {
    _instance = null;
  }

  /// scanForDevices
  /// this method is Future that  will call internal  ble api to start looking for available devices
  /// for period of time that can be modified using [secondsWait] that has default value which's 2 seconds
  /// the parameter [secondsWait] should be positive value or it will take default value
  ///
  /// this method returns nothing.
  ///
  /// this method throw [NotFoundAdapterSelectedException], if you don't configure correctly [BluetoothCore] instance
  ///
  /// [secondsWait] : (int) number of seconds that should be scanning for available devices
  Future scanForDevices({int secondsWait = 2});

  /// getListDevices
  /// this method will return list of available devices that founded after start scanning
  /// this method should be run after [scanForDevices] method to make sure that you will get list of devices
  ///
  /// return list of [Device] that represent the detail data of each available device
  ///
  /// this method throw [NotFoundAdapterSelectedException], if you don't configure correctly [BluetoothCore] instance
  Future<List<Device>> getListDevices();

  /// connect
  /// this method will connect to the device that has the address with parameter [deviceAddress],
  /// return bool that represent the state of the operation which means that if the result is true,your device connected succefully to the desired device
  /// if it false,the connection is failed
  ///
  /// this method doesn't throw any exception.
  ///
  /// accepet [deviceAddress] as parameter and return bool
  ///
  /// [deviceAddress] : (String) (required) this represent the address of the device that use want to connect to it
  Future<bool> connect({required String deviceAddress});

  /// disconnect
  /// this method will disconnect to the device that's already connected to it,if their is no device connected will throw exception,
  /// return bool that represent the state of the operation which means that if the result is true,your device disconnected succefully from the prvious device
  /// if it false,the disconnection is failed
  ///
  /// this method exception if their no device connected.
  Future<bool> disconnect();
}

class BluetoothCoreImpl extends BluetoothCore {
  BluetoothCoreImpl.setUp() : super._();

  @override
  Future scanForDevices({int secondsWait = 2}) async {
    final completer = Completer<int>();
    final ptr = _bleFFI.blePointer;
    final cachePTR = _bleFFI.bleCachePointer;
    final sendPort = singleCompletePort(completer);
    _bleFFI.scanForDevices(
      ptr,
      cachePTR,
      sendPort.nativePort,
      seconds: secondsWait,
    );
    final result = await completer.future;
    if (result == -1) {
      throw const NotFoundAdapterSelectedException();
    }
  }

  @override
  Future<List<Device>> getListDevices() async {
    final completer = Completer<String>();
    final ptr = _bleFFI.blePointer;
    final cachePTR = _bleFFI.bleCachePointer;
    final sendPort = singleCompletePort(completer);
    _bleFFI.getListDevices(
      cachePTR.value,
      sendPort.nativePort,
    );
    final resultJson = await completer.future;
    final res = resultJson;
    if (res.contains("err")) {
      throw const NotFoundAdapterSelectedException();
    }
    final List jsonDevice = jsonDecode(res);
    return (jsonDevice).map((e) => Device.fromMap(e)).toList();
  }

  @override
  Future<bool> connect({required String deviceAddress}) async {
    final completer = Completer<int>();
    final ptr = _bleFFI.blePointer;
    final cachePTR = _bleFFI.bleCachePointer;
    final sendPort = singleCompletePort(completer);
    _bleFFI.connectToDevice(
      ptr,
      cachePTR,
      sendPort.nativePort,
      deviceAddress,
    );
    final result = await completer.future;

    return result == 1 ? true : false;
  }

  @override
  Future<bool> disconnect() async {
    final completer = Completer<int>();
    final ptr = _bleFFI.blePointer;
    final cachePTR = _bleFFI.bleCachePointer;
    final sendPort = singleCompletePort(completer);
    _bleFFI.disconnect(
      ptr,
      cachePTR,
      sendPort.nativePort,
    );
    final result = await completer.future;
    if (result == -404) {
      throw Exception("there is no device connected before");
    }
    return result == 1 ? true : false;
  }
}
