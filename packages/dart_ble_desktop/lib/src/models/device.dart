class Device {
  final String nameDevice;
  final String addressDevice;
  final bool status;

  Device(
      {required this.nameDevice,
      required this.addressDevice,
      this.status = false});

  Device.fromMap(Map m)
      : nameDevice = m["name"],
        addressDevice = m["adr"],
        status = m["is_connected"];

  @override
  String toString() {
    return "{name:$nameDevice,address:$addressDevice,status:$status}";
  }
}
