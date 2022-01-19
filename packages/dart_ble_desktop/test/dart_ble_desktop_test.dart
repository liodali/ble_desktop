import 'package:flutter_test/flutter_test.dart';

import 'package:dart_ble_desktop/dart_ble_desktop.dart';

void main() async {
  await initBluetoothDesktop("dynamicLib");
  BluetoothCore bleCore = BluetoothCoreImpl.setUp();

  test("test get list ble desktop", () async {
    final devices = await bleCore.getListDevices(secondsWait: 1);
    final device = devices.firstWhere((e) => e.nameDevice.contains("WH-1000XM3"));
    expect(devices.isNotEmpty, true);
    expect(device.addressDevice, "38:18:4C:BE:EA:7C");
  });

  tearDown(() {
    bleCore.dispose();
  });
}
