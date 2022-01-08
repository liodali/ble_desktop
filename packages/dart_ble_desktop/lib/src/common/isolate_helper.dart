import 'dart:async';

import 'dart:isolate';

SendPort singleCompletePort<R, P>(
  Completer<R> completer, {
  FutureOr<R> Function(P message)? callback,
  Duration? timeout,
  FutureOr<R> Function()? onTimeout,
}) {
  if (callback == null && timeout == null) {
    return singleCallbackPort<Object>((response) {
      castComplete<R>(completer, response);
    });
  }
  var responsePort = RawReceivePort();
  Timer? timer;
  if (callback == null) {
    responsePort.handler = (response) {
      responsePort.close();
      timer?.cancel();
      castComplete<R>(completer, response);
    };
  } else {
    var zone = Zone.current;
    var action = zone.registerUnaryCallback((response) {
      try {
        // Also catch it if callback throws.
        completer.complete(callback(response as P));
      } catch (error, stack) {
        completer.completeError(error, stack);
      }
    });
    responsePort.handler = (response) {
      responsePort.close();
      timer?.cancel();
      zone.runUnary(action, response as P);
    };
  }
  if (timeout != null) {
    timer = Timer(timeout, () {
      responsePort.close();
      if (onTimeout != null) {
        /// workaround for incomplete generic parameters promotion.
        /// example is available in 'TimeoutFirst with invalid null' test
        try {
          completer.complete(Future.sync(onTimeout));
        } catch (e, st) {
          completer.completeError(e, st);
        }
      } else {
        completer
            .completeError(TimeoutException('Future not completed', timeout));
      }
    });
  }
  return responsePort.sendPort;
}

/// Helper function for [singleCallbackPort].
///
/// Replace [singleCallbackPort] with this
/// when removing the deprecated parameters.
SendPort singleCallbackPort<P>(void Function(P) callback) {
  var responsePort = RawReceivePort();
  var zone = Zone.current;
  callback = zone.registerUnaryCallback(callback);
  responsePort.handler = (response) {
    responsePort.close();
    zone.runUnary(callback, response as P);
  };
  return responsePort.sendPort;
}

// Helper function that casts an object to a type and completes a
// corresponding completer, or completes with the error if the cast fails.
void castComplete<R>(Completer<R> completer, Object? value) {
  try {
    completer.complete(value as R);
  } catch (error, stack) {
    completer.completeError(error, stack);
  }
}
