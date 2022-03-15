import 'dart:io';

import 'package:dart_ble_desktop/src/models/device.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:dart_ble_desktop/dart_ble_desktop.dart';

void main() async {
  await initBluetoothDesktop("dynamicLib");
  BluetoothCore bleCore = BluetoothCore.getInstance();
  const String adr = String.fromEnvironment("device-adr", defaultValue: "");
  const String name = String.fromEnvironment("device-name", defaultValue: "");
  await bleCore.scanForDevices(secondsWait: 1);
  
  group("test base method in ble core ", () {
    test("test get list ble desktop", () async {
      await Future.delayed(const Duration(milliseconds: 500), () async {});
      final devices = await bleCore.getListDevices();
      print(devices);
      final Device? device =
          devices.firstWhere((e) => e.nameDevice.contains(name));
      if (device == null) {
        assert(false, "device not found");
      }
      expect(devices.isNotEmpty, true);
      expect(device != null, true);
      if (adr.isEmpty) {
        assert(false, "should check env to set device-adr");
      }
      expect(device!.addressDevice, adr);
      expect(device.nameDevice, name);
    });

    test("test connection", () async {
      await Future.delayed(const Duration(milliseconds: 500), () async {});

      print("adr from dart : $adr");
      final isConnected = await bleCore.connect(deviceAddress: adr);
      expect(isConnected, true);
    });
    test("test disconnect", () async {
      await Future.delayed(const Duration(milliseconds: 500), () async {});
      final isConnected = await bleCore.disconnect();
      expect(isConnected, true);
    });
  });
}
