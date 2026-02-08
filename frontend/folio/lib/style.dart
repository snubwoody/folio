import 'package:flutter/cupertino.dart';

// BoxShadow(
// color: Colors.purple50,
// blurRadius: 10,
// offset: Offset(0, 4), // x and y axis offset
// )

// --shadow-purple-xs: 0 0 2px 0 rgb(75 25 226 / 0.05);
// --shadow-purple-sm:
// 0 1px 2px -1px rgb(75 25 226 / 0.1), 0 1px 3px 0px rgb(75 25 226 / 0.1);
// --shadow-purple-md:
// 0 2px 4px -2px rgb(75 25 226 / 0.1), 0 4px 6px -1px rgb(75 25 226 / 0.1);
// --shadow-purple-lg:
// 0 01px 15px -3px rgb(75 25 226 / 0.1),
// 0 4px 6px -4px rgb(75 25 226 / 0.1);
// --shadow-purple-xl:
// 0 20px 25px -5px rgb(75 25 226 / 0.1),
// 0 8px 10px -6px rgb(75 25 226 / 0.1);

abstract final class Style {
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
