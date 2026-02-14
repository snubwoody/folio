import 'package:flutter/material.dart' hide Colors, IconButton;
import 'package:folio/colors.dart';
import 'package:folio/components.dart';
import 'package:folio/state.dart';
import 'package:provider/provider.dart';

class TransactionPanel extends StatelessWidget {
  const TransactionPanel({super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
      spacing: 24,
      children: [
        Row(children: [TextLabel("Transactions"), Spacer(), TextLabel("New")]),
        TransactionTable(),
      ],
    );
  }
}

class TransactionTable extends StatelessWidget {
  const TransactionTable({super.key});

  @override
  Widget build(BuildContext context) {
    final expenses = context.watch<TransactionsStore>().expenses;
    return Table(
      children: [
        ...expenses.map((e) => TableRow(
          children: [
            TextCell(e.account?.name ?? ""),
            TextCell(e.category?.title ?? ""),
            TextCell("${e.amount}"),
            TextCell("${e.date.toString()}"),
          ]
        )),
        TableRow(children: [
          // Stack(children: [
          //   Positioned(child: TextCell("text"),left: -23,)
          // ],),
          TextCell("Hi"),
          TextCell("Hey"),
          TextCell("Hi"),
          TextCell("Hi"),
        ])
      ],
    );
  }
}

class TextCell extends StatelessWidget {
  final String text;
  const TextCell(this.text, {super.key});

  @override
  Widget build(BuildContext context) {
    return TableCell(child: TextLabel(text));
  }
}

class DropdownCell extends StatelessWidget {
  const DropdownCell({super.key});

  @override
  Widget build(BuildContext context) {
    return DropdownMenu(
      dropdownMenuEntries: [
        DropdownMenuEntry(value: "1", label: "Account 1"),
        DropdownMenuEntry(value: "2", label: "Account 2"),
        DropdownMenuEntry(value: "3", label: "Account 3"),
      ],
    );
  }
}

class TableCell extends StatelessWidget {
  final Widget child;
  const TableCell({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: Container(
        padding: EdgeInsets.all(12),
        decoration: BoxDecoration(
          border: Border.all(color: Colors.neutral50, width: 1),
        ),
        child: child,
      ),
    );
  }
}
