class Device {
  final String nameDevice;
  final String addressDevice;

  Device({
    required this.nameDevice,
    required this.addressDevice,
  });

  Device.fromMap(Map m)
      : nameDevice = m["name"],
        addressDevice = m["adr"];
}
