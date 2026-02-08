import 'package:flutter/material.dart' hide Colors, IconButton;
import 'package:folio/colors.dart';
import 'package:folio/components.dart';
import 'package:folio/state.dart';
import 'package:folio/style.dart';
import 'package:lucide_icons_flutter/lucide_icons.dart';
import 'package:provider/provider.dart';

import 'home/transaction_table.dart';

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
      spacing: 24,
      children: [
        AccountPanel(),
        Expanded(child: TransactionPanel()),
      ],
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
        Row(children: [TextLabel("Transactions"), Spacer(), TextLabel("New")]),
        TransactionTable(),
      ],
    );
  }
}

class AccountPanel extends StatelessWidget {
  const AccountPanel({super.key});

  @override
  Widget build(BuildContext context) {
    final accounts = context.watch<AccountStore>().accounts;
    final accountStore = context.read<AccountStore>();
    return Column(
      children: [
        Row(
          children: [
            TextLabel("Accounts"),
            Spacer(),
            IconButton(
              icon: LucideIcons.plus,
              onTap: () =>
                  accountStore.addAccount(Account(name: "New account")),
            ),
          ],
        ),
        Row(
          spacing: 24,
          children: accounts
              .map(
                (account) =>
                    AccountCard(name: account.name, balance: account.balance),
              )
              .toList(),
        ),
      ],
    );
  }
}

class AccountCard extends StatelessWidget {
  final String name;
  final double balance;
  const AccountCard({super.key, this.name = "", this.balance = 0});

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
