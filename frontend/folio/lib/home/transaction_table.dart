import 'package:flutter/material.dart' hide Colors, IconButton;
import 'package:folio/colors.dart';
import 'package:folio/components.dart';
import 'package:folio/main.dart';
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
    final accounts = context.watch<AccountStore>().accounts;
    final categories = context.watch<SettingsStore>().categories;
    print(accounts);
    return Table(
      border: TableBorder.all(color: Colors.borderNeutral50),
      children: [
        ...expenses.map((e) => TableRow(
          children: [
            DropdownCell(
              initialValue: e.account?.name,
              entries: accounts.map((a) => DropdownMenuEntry(value: a.id, label: a.name)).toList(),
            ),
            DropdownCell(
              initialValue: e.category?.title,
              entries: categories.map((a) => DropdownMenuEntry(value: a.id, label: a.title)).toList(),
            ),
            TextCell("${e.amount}"),
            TextCell("${e.date.toString()}"),
          ]
        )),
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

// TODO: padding
class DropdownCell extends StatelessWidget {
  final String? initialValue;
  final List<DropdownMenuEntry> entries ;
  const DropdownCell({super.key,required this.entries,this.initialValue});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: double.infinity,
      child: DropdownMenu(
        onSelected: (val){
        },
        initialSelection: initialValue,
        enableFilter: true,
        // width: double.infinity,
        dropdownMenuEntries: entries,
      ),
    );
  }
}

class TableCell extends StatelessWidget {
  final Widget child;
  const TableCell({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: EdgeInsets.all(12),
      // decoration: BoxDecoration(
        // border: Border.all(color: Colors.neutral50, width: 1),
      // ),
      child: child,
    );
  }
}
