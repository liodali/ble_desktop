import 'package:dart_ble_desktop/src/models/device.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:dart_ble_desktop/dart_ble_desktop.dart';

void main() async {
  await initBluetoothDesktop("dynamicLib");
  BluetoothCore bleCore = BluetoothCoreImpl.setUp();

  test("test get list ble desktop", () async {
    final devices = await bleCore.getListDevices(secondsWait: 1);
    const String name = String.fromEnvironment("device-name", defaultValue: "");

    final Device? device =
        devices.firstWhere((e) => e.nameDevice.contains(name));
    if (device == null) {
      assert(false, "device not found");
    }
    expect(devices.isNotEmpty, true);
    expect(device != null, true);
    String adr = const String.fromEnvironment("device-adr", defaultValue: "");
    if (adr.isEmpty) {
      assert(false, "should check env to set device-adr");
    }
    expect(device!.addressDevice, adr);
    expect(device.nameDevice, name);
  });

  tearDown(() {
    bleCore.dispose();
  });
}
