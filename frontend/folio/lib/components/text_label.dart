import 'package:flutter/material.dart';

class TextLabel extends StatelessWidget {
  final String text;
  const TextLabel(this.text, {super.key});

  @override
  Widget build(BuildContext context) {
    return Text(text);
  }
}
