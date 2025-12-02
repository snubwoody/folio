import 'package:flutter/material.dart' hide Colors;
import 'package:folio/colors.dart';

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
            onSurface: Colors.black
        )
      ),
      home: Scaffold(body: const ExpenseTable()),
    );
  }
}

class ExpenseTable extends StatelessWidget {
  const ExpenseTable({super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
        children: [
          TableHeader(),
          TableRow(),
          TableRow(),
          TableRow(),
          TableRow(),
        ],
    );
  }
}

class TableHeader extends StatelessWidget {
  const TableHeader({super.key});

  @override
  Widget build(BuildContext context) {
    return const Row(
      children: [
        TableCell(),
        TableCell(),
        TableCell(),
        TableCell(),
      ],
    );
  }
}

class TableRow extends StatelessWidget {
  const TableRow({super.key});

  @override
  Widget build(BuildContext context) {
    return const Row(
      children: [
        TableCell(),
        TableCell(),
        TableCell(),
        TableCell(),
      ],
    );
  }
}

class TableCell extends StatelessWidget {
  const TableCell({super.key});

  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: Container(
          padding: EdgeInsets.all(12),
          decoration: BoxDecoration(
            border: BoxBorder.fromBorderSide(BorderSide(color: Colors.borderNeutral50))
          ),
          child: const TextLabel("Cell")
      ),
    );
  }
}

class TextLabel extends StatelessWidget {
  final String value;
  const TextLabel(this.value,{super.key});

  @override
  Widget build(BuildContext context) {
    return Text(value);
  }
}




