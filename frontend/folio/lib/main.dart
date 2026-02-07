import 'package:flutter/material.dart' hide Colors;
import 'package:folio/colors.dart';
import 'package:folio/home_page.dart';
import 'package:lucide_icons_flutter/lucide_icons.dart';

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
      home: Scaffold(
        body: Row(
          children: const [
            NavigationPanel(),
            Expanded(
              child: Padding(padding: EdgeInsets.all(24.0), child: HomePage()),
            ),
          ],
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
      padding: EdgeInsets.symmetric(vertical: 24,horizontal: 12),
      color: Colors.neutral25,
      child: const Column(
        spacing: 24,
        children: [
          IconButton(icon: LucideIcons.house),
          IconButton(icon: LucideIcons.chartArea),
          Spacer(),
          IconButton(icon: LucideIcons.settings),
        ],
      ),
    );
  }
}

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

class ExpenseTable extends StatelessWidget {
  const ExpenseTable({super.key});

  @override
  Widget build(BuildContext context) {
    return Column(children: []);
  }
}
