class NotFoundAdapterSelectedException implements Exception {
  const NotFoundAdapterSelectedException();

   @override
  String toString() {
    return "Exception No Adapter Found: There are no adapter selected or found";
  }
}
