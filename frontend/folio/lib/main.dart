import 'package:flutter/material.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        colorScheme: ColorScheme(
            brightness: Brightness.light,
            primary: Colors.blue,
            onPrimary: Colors.white,
            secondary: Colors.purple,
            onSecondary: Colors.white,
            error: Colors.red,
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
    return GridView.count(
      crossAxisCount: 4,
      mainAxisSpacing: 0,
      children: [
        TableCell(),
        TableCell(),
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
    return Container(
        child: const TextLabel("Cell")
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




