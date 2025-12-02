import 'dart:ui';

abstract final class Colors{
  /// Completely transparent
  static const Color transparent = Color(0x00000000);

  static const Color black = Color(0xFF000000);
  static const Color white= Color(0xFFFFFFFF);

  // Neutral tones
  static const Color neutral25 = Color(0xFFFCFCFC);
  static const Color neutral50 = Color(0xFFF7F7F7);
  static const Color neutral100 = Color(0xFFE6E6E6);
  static const Color neutral200 = Color(0xFFCBCBCB);
  static const Color neutral300 = Color(0xFFBEBEBE);
  static const Color neutral400 = Color(0xFF9E9E9E);
  static const Color neutral500 = Color(0xFF808080);
  static const Color neutral600 = Color(0xFF787878);
  static const Color neutral700 = Color(0xFF626262);
  static const Color neutral800 = Color(0xFF3E3E3E);
  static const Color neutral900 = Color(0xFF282828);
  static const Color neutral950 = Color(0xFF171717);

  // Purple tones
  static const Color purple50 = Color(0xFFEBE9FE);
  static const Color purple100 = Color(0xFFD9D6FD);
  static const Color purple200 = Color(0xFFB2AAFB);
  static const Color purple300 = Color(0xFF8F80F9);
  static const Color purple400 = Color(0xFF6D52F6);
  static const Color purple500 = Color(0xFF4B19E2);
  static const Color purple600 = Color(0xFF3D13BC);
  static const Color purple700 = Color(0xFF2E0C94);
  static const Color purple800 = Color(0xFF20076E);
  static const Color purple900 = Color(0xFF14034E);
  static const Color purple950 = Color(0xFF0A0133);

  // Text colors
  static const Color textHeading = Colors.neutral950;
  static const Color textBody = Colors.neutral900;
  static const Color textMuted = Colors.neutral600;

  // Border colors
  static const Color borderNeutral50 = Colors.neutral50;
  static const Color borderFocus = Colors.purple500;
}