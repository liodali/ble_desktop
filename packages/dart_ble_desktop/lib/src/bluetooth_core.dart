import 'dart:async';
import 'dart:convert';
import 'dart:isolate';
import 'package:dart_ble_desktop/src/models/device.dart';
import 'package:ffi/ffi.dart';
import 'dart:ffi';
import 'package:flutter/material.dart';

import 'common/isolate_helper.dart';
import 'native/ble_ffi.dart';

int currentIdBleCore = 0;

abstract class BluetoothCore {
  late BleFFI bleFFI;

  int _idBleCore = currentIdBleCore;

  static final _instances = <int, BluetoothCore>{};

  static BluetoothCore? getInstance({int idBleCore = -1}) {
    var id = idBleCore;
    if (idBleCore == -1) {
      id = currentIdBleCore;
    }
    return _instances[id];
  }

  static init(String namePathLib) {
    BleFFI.init(namePathLib);
  }

  BluetoothCore.setUp() {
    _idBleCore++;
    bleFFI = BleFFI.instance;
    _instances[_idBleCore] = this;
  }
  @mustCallSuper
  dispose() {
    currentIdBleCore--;
  }

  Future<List<Device>> getListDevices({int secondsWait = 2});
}

class BluetoothCoreImpl extends BluetoothCore {
  BluetoothCoreImpl.setUp() : super.setUp();

  @override
  Future<List<Device>> getListDevices({int secondsWait = 2}) async {
    final completer = Completer<String>();
    final sendPort = singleCompletePort(completer);
    bleFFI.getListDevices(sendPort.nativePort,seconds: secondsWait);
    final resultJson = await completer.future;
    print(resultJson);
    final List<Map> jsonDevice = json.decode(resultJson);
    return jsonDevice.map((e) => Device.fromMap(e)).toList();
  }
}
