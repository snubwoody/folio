import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';

/// Base button that provides basic button functionality such as
/// tap callbacks. It is intended to be used as a building block for
/// other buttons.
class BaseButton extends StatefulWidget {
  /// The color to fill the background of the button
  final Color? color;
  /// The color to fill the background of the button while
  /// the cursor is over the button.
  final Color? hoverColor;
  final BorderRadiusGeometry? borderRadius;
  final EdgeInsetsGeometry? padding;
  final void Function()? onTap;
  final Widget child;

  const BaseButton({
    super.key,
    required this.child,
    this.color,
    this.onTap,
    this.padding,
    this.borderRadius,
    this.hoverColor
  });

  @override
  State<BaseButton> createState() => _BaseButtonState();
}

class _BaseButtonState extends State<BaseButton> {
  bool _hovered = false;

  void setHovered(bool hovered){
    setState(() {
      _hovered = hovered;
    });
  }

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      onEnter: (PointerEnterEvent e) => setHovered(true),
      onExit: (PointerExitEvent e) => setHovered(false),
      cursor: SystemMouseCursors.click,
      child: GestureDetector(
        onTap: () => widget.onTap?.call(),
        child: Container(
          padding: widget.padding,
          decoration: BoxDecoration(
            borderRadius: widget.borderRadius,
            // TODO: animate
            color: _hovered ? widget.hoverColor : widget.color
          ),
          child: widget.child,
        ),
      ),
    );;
  }
}