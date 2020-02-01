library flutter_qrcode_plugin;

import 'dart:async';
import 'package:flutter/services.dart';
import 'package:meta/meta.dart';

class FlutterQrcodePlugin {}

class QrCodeController {
  EventChannel events = EventChannel('rust/qrcode');
  List<String> data = [];
  StreamSubscription sub;

  void Function() onStateChange;
  void Function(String) onDetected;

  bool isInitialized = false;
  int textureId;
  double aspectRatio;
  int width;
  int height;

  QrCodeController(@required this.onStateChange, @required this.onDetected);

  listen() {
    final stream = events.receiveBroadcastStream();
    sub = stream.listen(onData, onDone: onDone, onError: onError);
  }

  onData(dynamic data) {
    final initialized = data['initialized'];
    if (initialized != null) {
      textureId = initialized['textureId'];
      width = initialized['width'];
      height = initialized['height'];
      aspectRatio = width.toDouble() / height.toDouble();
      isInitialized = true;
      onStateChange();
      return;
    }
    final qrcode = data['qrCode'];
    if (qrcode != null) {
      onDetected(qrcode);
      return;
    }
    final disposed = data['disposed'];
    if (disposed != null) {
      isInitialized = false;
      textureId = null;
      width = null;
      height = null;
      aspectRatio = null;
      onStateChange();
      return;
    }
  }

  onError(dynamic error) {
    print('onError $error');
  }

  onDone() {
    print('onDone');
  }

  cancelStream() async {
    if (sub != null) {
      await sub.cancel();
      sub = null;
    }
  }

  void dispose() {
    cancelStream();
  }
}
