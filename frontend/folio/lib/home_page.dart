import 'package:flutter/material.dart' hide Colors;
import 'package:folio/colors.dart';
import 'package:folio/components.dart';
import 'package:folio/style.dart';

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(24.0),
      child: Column(spacing: 24,children: [AccountPanel(),Expanded(child: TransactionPanel())]),
    );
  }
}

class TransactionPanel extends StatelessWidget {
  const TransactionPanel({super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
      spacing: 24,
      children: [
        Row(children: [TextLabel("Transactions"), Spacer(),TextLabel("New")]),
        TransactionTable(),
      ],
    );
  }
}

class TransactionTableSource extends DataTableSource{
  @override
  int get rowCount => 2;

  @override
  DataRow? getRow(int index) {
    switch (index){
      case 0:
        return DataRow(cells: [
          DataCell(TextLabel("Name")),
          DataCell(TextLabel("RBC")),
          DataCell(TextLabel("2025-12-12")),
          DataCell(TextLabel("500.00")),
        ]);
        case 1:
        return DataRow(cells: [
          DataCell(TextLabel("Name")),
          DataCell(TextLabel("RBC")),
          DataCell(TextLabel("2025-12-12")),
          DataCell(TextLabel("500.00")),
        ]);
      default:
        return null;
    }
  }

  @override
  bool get isRowCountApproximate => false;

  @override
  int get selectedRowCount => 0;
}

class TransactionTable extends StatelessWidget {
  const TransactionTable({super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Row(
          children: [TableCell(),TableCell(),TableCell(),TableCell()],
        ),
        Row(
          children: [TableCell(),TableCell(),TableCell(),TableCell()],
        ),
        Row(
          children: [TableCell(),TableCell(),TableCell(),TableCell()],
        ),
      ],
    );
  }
}


class TableCell extends StatelessWidget {
  const TableCell({super.key});

  @override
  Widget build(BuildContext context) {
    return Expanded(child:
    Container(
      padding: EdgeInsets.all(12),
      decoration: BoxDecoration(
        border: Border.all(color: Colors.neutral50, width: 1)
      ),
        child: TextLabel("Data")
    ));
  }
}

class AccountPanel extends StatelessWidget {
  const AccountPanel({super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Row(children: [TextLabel("Accounts"), Spacer(), TextLabel("New")]),
        Row(
          spacing: 24,
          children: [
            Account(name: "RBC", balance: 24),
            Account(name: "BOC", balance: 224),
            Account(),
            Account(),
          ],
        ),
      ],
    );
  }
}

class Account extends StatelessWidget {
  final String name;
  final double balance;
  const Account({super.key, this.name = "", this.balance = 0});

  @override
  Widget build(BuildContext context) {
    // FIXME: make it fill with a max with
    return ConstrainedBox(
      constraints: BoxConstraints(maxWidth: double.infinity),
      child: Container(
        padding: EdgeInsets.all(16),
        decoration: BoxDecoration(
          color: Colors.white,
          borderRadius: BorderRadius.circular(12),
          boxShadow: Style.shadowPurpleMd,
        ),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [TextLabel(name), TextLabel("$balance")],
        ),
      ),
    );
  }
}
