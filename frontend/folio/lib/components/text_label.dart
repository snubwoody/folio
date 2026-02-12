import 'package:flutter/material.dart';

// TODO: add text styles
class TextLabel extends StatelessWidget {
  final String text;
  final Color? color;
  const TextLabel(this.text, {super.key, this.color});

  @override
  Widget build(BuildContext context) {
    return Text(text, style: TextStyle(color: color, height: 1.5));
  }
}
