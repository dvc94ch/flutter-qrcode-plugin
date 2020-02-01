import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter/foundation.dart'
    show debugDefaultTargetPlatformOverride;
import 'package:flutter_qrcode_plugin/flutter_qrcode_plugin.dart';

void main() {
  // Override is necessary to prevent 'Unknown platform' flutter startup error.
  debugDefaultTargetPlatformOverride = TargetPlatform.android;
  runApp(QrCodeApp());
}

class QrCodeApp extends StatefulWidget {
  @override
  _QrCodeAppState createState() => _QrCodeAppState();
}

class _QrCodeAppState extends State<QrCodeApp> {
  QrCodeController _controller;
  String _code = '';

  @override
  void initState() {
    super.initState();
    _controller = QrCodeController(
      () => setState(() {}),
      (code) => setState(() { _code = code; _controller?.dispose(); }),
    )
    ..listen();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'QrCode Demo',
      home: Scaffold(
        body: Center(
          child: Column(
            children: [
              _controller?.isInitialized ?? false
                ? AspectRatio(
                    aspectRatio: _controller.aspectRatio,
                    child: Texture(textureId: _controller.textureId),
                  )
                : Container(),
              Text(_code),
            ],
          ),
        ),
      ),
    );
  }

  @override
  void dispose() {
    super.dispose();
    _controller?.dispose();
  }
}
