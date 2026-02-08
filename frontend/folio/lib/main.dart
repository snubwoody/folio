import 'package:flutter/material.dart' hide Colors, IconButton;
import 'package:folio/colors.dart';
import 'package:folio/home_page.dart';
import 'package:folio/components.dart';
import 'package:folio/settings_panel.dart';
import 'package:folio/state.dart';
import 'package:lucide_icons_flutter/lucide_icons.dart';
import 'package:provider/provider.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});
  @override
  Widget build(BuildContext context) {
    return MultiProvider(
      providers: [
        ChangeNotifierProvider(
          create: (BuildContext context) => AccountStore(),
        ),
        ChangeNotifierProvider(
          create: (BuildContext context) => SettingsStore(),
        ),
      ],
      child: MaterialApp(
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
        home: Scaffold(
          body: Row(
            children: const [
              NavigationPanel(),
              Expanded(
                child: Padding(
                  padding: EdgeInsets.all(24.0),
                  child: HomePage(),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class NavigationPanel extends StatelessWidget {
  const NavigationPanel({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: EdgeInsets.symmetric(vertical: 24, horizontal: 12),
      color: Colors.neutral25,
      child: Column(
        spacing: 24,
        children: [
          const IconButton(icon: LucideIcons.house),
          const IconButton(icon: LucideIcons.chartArea),
          const Spacer(),
          IconButton(
            icon: LucideIcons.settings,
            onTap: () => showDialog(
              context: context,
              builder: (BuildContext context) => SettingsPanel(),
            ),
          ),
        ],
      ),
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
