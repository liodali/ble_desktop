import 'package:flutter_test/flutter_test.dart';

import 'package:dart_ble_desktop/dart_ble_desktop.dart';

void main() async {
  await initBluetoothDesktop("dynamicLib");
  BluetoothCore bleCore = BluetoothCoreImpl.setUp();

  test("test get list ble desktop", () async {
    final devices = await bleCore.getListDevices(secondsWait: 1);
    expect(devices.isNotEmpty, true);
  });

  tearDown(() {
    bleCore.dispose();
  });
}
