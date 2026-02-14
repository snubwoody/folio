import 'package:dropdown_button2/dropdown_button2.dart';
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
    return Table(
      border: TableBorder.all(color: Colors.borderNeutral50),
      children: [
        ...expenses.map((e) => TableRow(
          children: [
            DropdownCell(
              initialValue: e.account?.id,
              entries: accounts.map((a) => DropdownMenuEntry(value: a.id, label: a.name)).toList(),
            ),
            DropdownCell(
              initialValue: e.category?.id,
              entries: categories.map((c) => DropdownMenuEntry(value: c.id, label: c.title)).toList(),
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
class DropdownCell extends StatefulWidget {
  const DropdownCell({super.key,required this.entries,this.initialValue});

  final String? initialValue;
  final List<DropdownMenuEntry> entries ;

  List<DropdownMenuItem> get menuItems {
    return entries.map((e) => DropdownMenuItem(value:e.value,child: TextLabel(e.label))).toList();
  }

  @override
  State<DropdownCell> createState() => _DropdownCellState();
}

class _DropdownCellState extends State<DropdownCell> {
  String? _selectedValue;

  @override
  void initState() {
    super.initState();
    _selectedValue = widget.initialValue;
    // final hasItem = widget.menuItems.firstWhere((item) => item.value == "") != null;
    // if (hasItem){
    // }
    // widget.menuItems.forEach((i) => print("Menu item id=${i.value}"));
  }

  @override
  Widget build(BuildContext context) {
    return DropdownButtonHideUnderline(
      child: DropdownButton2(
        isExpanded: true,
        value: _selectedValue,
        onChanged: (value){
            setState(() {
              _selectedValue = value;
            });
        },
        items: widget.menuItems,
        buttonStyleData: ButtonStyleData(elevation: 0,),
        dropdownStyleData: DropdownStyleData(),
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
