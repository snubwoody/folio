import 'package:flutter/material.dart';

class IconButton extends StatefulWidget {
  final void Function()? onTap;
  final IconData icon;
  const IconButton({super.key, this.onTap, required this.icon});

  @override
  State<IconButton> createState() => _IconButtonState();
}

class _IconButtonState extends State<IconButton> {
  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      cursor: SystemMouseCursors.click,
      child: GestureDetector(
        onTap: () {
          widget.onTap?.call();
        },
        child: Icon(widget.icon),
      ),
    );
  }
}
