import 'package:flutter/material.dart';
import 'package:folio/style.dart';

// TODO: lose focus
// TODO: dispose controller
class InlineTextField extends StatelessWidget {
  final TextEditingController? controller;
  final void Function(String)? onSubmitted;
  const InlineTextField({super.key, this.controller, this.onSubmitted});

  void _submit(String value) {
    onSubmitted?.call(value);
  }

  @override
  Widget build(BuildContext context) {
    return TextFormField(
      controller: controller,
      onFieldSubmitted: _submit,
      style: TextStyles.body,
      decoration: InputDecoration(
        border: InputBorder.none,
        focusedBorder: UnderlineInputBorder(),
      ),
    );
  }
}
