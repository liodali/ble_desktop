import 'package:flutter_test/flutter_test.dart';

import 'package:dart_ble_desktop/dart_ble_desktop.dart';

void main() async{
  await initBluetoothDesktop("dynamicLib");
  test("test ble desktop", () async {
    BluetoothCore bleCore = BluetoothCoreImpl.setUp();
    final devices = await bleCore.getListDevices();
    assert(devices.isNotEmpty, true);
  });
}
