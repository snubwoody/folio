import 'package:flutter/material.dart' hide Colors;
import 'package:folio/colors.dart';
import 'package:folio/home_page.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'Folio',
      theme: ThemeData(
        colorScheme: ColorScheme(
          brightness: Brightness.light,
          primary: Colors.purple500,
          onPrimary: Colors.white,
          secondary: Colors.purple500,
          onSecondary: Colors.white,
          error: Colors.purple500,
          onError: Colors.white,
          surface: Colors.white,
          onSurface: Colors.black,
        ),
      ),
      home: Scaffold(body: const HomePage()),
    );
  }
}

class NavigationPanel extends StatelessWidget {
  const NavigationPanel({super.key});

  @override
  Widget build(BuildContext context) {
    return const Column(
      children: [Text("Home"), Text("Analytics"), Text("Settings")],
    );
  }
}

class ExpenseTable extends StatelessWidget {
  const ExpenseTable({super.key});

  @override
  Widget build(BuildContext context) {
    return Column(children: []);
  }
}
