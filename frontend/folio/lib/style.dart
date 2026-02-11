import 'package:flutter/material.dart';

// TODO: add font
abstract final class TextStyles{
  static const String _fontFamily = "Satoshi";

  static const TextStyle h1 = TextStyle(
    fontSize: Style.textSizeH1,
    fontVariations: [FontVariation("wght", 400)],
    height:  1.0
  );
  static const TextStyle h2= TextStyle(
    fontFamily: _fontFamily,
    fontSize: Style.textSizeH2,
    fontVariations: [FontVariation("wght", 400)],
    height:  1.0
  );
  static const TextStyle h3= TextStyle(
    fontFamily: _fontFamily,
    fontSize: Style.textSizeH3,
    fontVariations: [FontVariation("wght", 400)],
    height:  1.0
  );
  static const TextStyle h4= TextStyle(
    fontFamily: _fontFamily,
    fontSize: Style.textSizeH4,
    fontVariations: [FontVariation("wght", 400)],
    height:  1.25
  );
  static const TextStyle h5= TextStyle(
    fontFamily: _fontFamily,
    fontSize: Style.textSizeH5,
    fontVariations: [FontVariation("wght", 400)],
    height:  1.25
  );
  static const TextStyle h6 = TextStyle(
    fontFamily: _fontFamily,
    fontSize: Style.textSizeH6,
    fontVariations: [FontVariation("wght", 400)],
    height:  1.5
  );
  static const TextStyle body = TextStyle(
    fontFamily: _fontFamily,
    fontSize: Style.textSizeBody,
    fontWeight: FontWeight.w400,
    fontVariations: [FontVariation("wght", 400)],
    height:  1.5
  );
  static const TextStyle small = TextStyle(
    fontFamily: _fontFamily,
    fontSize: Style.textSizeSm,
    fontVariations: [FontVariation("wght", 400)],
    height:  1.5
  );
  static const TextStyle extraSmall = TextStyle(
    fontFamily: _fontFamily,
    fontSize: Style.textSizeXs,
    fontVariations: [FontVariation("wght", 400)],
    height:  1.5
  );
}

abstract final class Style {
  static const double textSizeH1 = 64;
  static const double textSizeH2 = 56;
  static const double textSizeH3 = 48;
  static const double textSizeH4 = 36;
  static const double textSizeH5 = 25;
  static const double textSizeH6 = 18;
  static const double textSizeBody = 16;
  static const double textSizeSm = 13;
  static const double textSizeXs = 10;

  static const List<BoxShadow> shadowXs = [
    BoxShadow(color: Color.fromRGBO(0, 0, 0, 0.05), blurRadius: 2),
  ];

  static const List<BoxShadow> shadowSm = [
    BoxShadow(
      color: Color.fromRGBO(0, 0, 0, 0.1),
      blurRadius: 2,
      spreadRadius: -1,
      offset: Offset(0, 1),
    ),
    BoxShadow(
      color: Color.fromRGBO(0, 0, 0, 0.1),
      blurRadius: 3,
      offset: Offset(0, 1),
    ),
  ];

  static const List<BoxShadow> shadowMd = [
    BoxShadow(
      color: Color.fromRGBO(0, 0, 0, 0.1),
      blurRadius: 4,
      spreadRadius: -2,
      offset: Offset(0, 2),
    ),
    BoxShadow(
      color: Color.fromRGBO(0, 0, 0, 0.1),
      blurRadius: 6,
      spreadRadius: -1,
      offset: Offset(0, 4),
    ),
  ];

  static const List<BoxShadow> shadowLg = [
    BoxShadow(
      color: Color.fromRGBO(0, 0, 0, 0.1),
      blurRadius: 15,
      spreadRadius: -3,
      offset: Offset(0, 1),
    ),
    BoxShadow(
      color: Color.fromRGBO(0, 0, 0, 0.1),
      blurRadius: 6,
      spreadRadius: -4,
      offset: Offset(0, 4),
    ),
  ];

  static const List<BoxShadow> shadowXl = [
    BoxShadow(
      color: Color.fromRGBO(0, 0, 0, 0.1),
      blurRadius: 25,
      spreadRadius: -5,
      offset: Offset(0, 20),
    ),
    BoxShadow(
      color: Color.fromRGBO(0, 0, 0, 0.1),
      blurRadius: 10,
      spreadRadius: -6,
      offset: Offset(0, 8),
    ),
  ];

  static const List<BoxShadow> shadowPurpleXs = [
    BoxShadow(color: Color.fromRGBO(75, 25, 226, 0.05), blurRadius: 2),
  ];

  static const List<BoxShadow> shadowPurpleSm = [
    BoxShadow(
      color: Color.fromRGBO(75, 25, 226, 0.1),
      blurRadius: 2,
      spreadRadius: -1,
      offset: Offset(0, 1),
    ),
    BoxShadow(
      color: Color.fromRGBO(75, 25, 226, 0.1),
      blurRadius: 3,
      offset: Offset(0, 1),
    ),
  ];

  static const List<BoxShadow> shadowPurpleMd = [
    BoxShadow(
      color: Color.fromRGBO(75, 25, 226, 0.1),
      blurRadius: 4,
      spreadRadius: -2,
      offset: Offset(0, 2),
    ),
    BoxShadow(
      color: Color.fromRGBO(75, 25, 226, 0.1),
      blurRadius: 6,
      spreadRadius: -1,
      offset: Offset(0, 4),
    ),
  ];

  static const List<BoxShadow> shadowPurpleLg = [
    BoxShadow(
      color: Color.fromRGBO(75, 25, 226, 0.1),
      blurRadius: 15,
      spreadRadius: -3,
      offset: Offset(0, 1),
    ),
    BoxShadow(
      color: Color.fromRGBO(75, 25, 226, 0.1),
      blurRadius: 6,
      spreadRadius: -4,
      offset: Offset(0, 4),
    ),
  ];

  static const List<BoxShadow> shadowPurpleXl = [
    BoxShadow(
      color: Color.fromRGBO(75, 25, 226, 0.1),
      blurRadius: 25,
      spreadRadius: -5,
      offset: Offset(0, 20),
    ),
    BoxShadow(
      color: Color.fromRGBO(75, 25, 226, 0.1),
      blurRadius: 10,
      spreadRadius: -6,
      offset: Offset(0, 8),
    ),
  ];
}
