import 'package:flutter/material.dart';
import 'package:folio/style.dart';


class InlineTextField extends StatelessWidget {
  final TextEditingController? controller;
  const InlineTextField({super.key, this.controller});

  @override
  Widget build(BuildContext context) {
    return TextFormField(
      controller: controller,
      style: TextStyles.body,
      decoration: InputDecoration(
        border: InputBorder.none,
        focusedBorder: UnderlineInputBorder()
      ),
    );
  }
}
