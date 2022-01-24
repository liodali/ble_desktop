import 'package:dart_ble_desktop/src/models/device.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:dart_ble_desktop/dart_ble_desktop.dart';

void main() async {
  await initBluetoothDesktop("dynamicLib");
  BluetoothCore bleCore = BluetoothCoreImpl.setUp();
  const String adr = String.fromEnvironment("device-adr", defaultValue: "");
  const String name = String.fromEnvironment("device-name", defaultValue: "");

  test("test get list ble desktop", () async {
    await bleCore.scanForDevices(secondsWait: 1);
    final devices = await bleCore.getListDevices();
    print(devices);
    final Device? device = devices.firstWhere((e) => e.nameDevice == name);
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
    print("adr from dart : $adr");
    final isConnected = await bleCore.connect(deviceAddress: adr);
    expect(isConnected, true);
  });

  tearDown(() {
    bleCore.dispose();
  });
}
