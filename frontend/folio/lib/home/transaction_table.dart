import 'package:flutter/material.dart' hide Colors, IconButton;
import 'package:folio/colors.dart';
import 'package:folio/components.dart';
import 'package:folio/state.dart';
import 'package:folio/style.dart';
import 'package:lucide_icons_flutter/lucide_icons.dart';
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

// class TransactionTableSource extends DataTableSource {
//   @override
//   int get rowCount => 2;
//
//   @override
//   DataRow? getRow(int index) {
//     switch (index) {
//       case 0:
//         return DataRow(
//           cells: [
//             DataCell(TextLabel("Name")),
//             DataCell(TextLabel("RBC")),
//             DataCell(TextLabel("2025-12-12")),
//             DataCell(TextLabel("500.00")),
//           ],
//         );
//       case 1:
//         return DataRow(
//           cells: [
//             DataCell(TextLabel("Name")),
//             DataCell(TextLabel("RBC")),
//             DataCell(TextLabel("2025-12-12")),
//             DataCell(TextLabel("500.00")),
//           ],
//         );
//       default:
//         return null;
//     }
//   }
//
//   @override
//   bool get isRowCountApproximate => false;
//
//   @override
//   int get selectedRowCount => 0;
// }

class TransactionTable extends StatelessWidget {
  const TransactionTable({super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Row(children: [DropdownCell(), TextCell("Data"), TextCell("Data"), TextCell("Data")]),
        Row(children: [DropdownCell(), TextCell("Data"), TextCell("Data"), TextCell("Data")]),
        Row(children: [DropdownCell(), TextCell("Data"), TextCell("Data"), TextCell("Data")]),
      ],
    );
  }
}

class TextCell extends StatelessWidget {
  final String text;
  const TextCell(this.text,{super.key});

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
        ]
    );
  }
}



class TableCell extends StatelessWidget {
  final Widget child;
  const TableCell({super.key,required this.child});

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



