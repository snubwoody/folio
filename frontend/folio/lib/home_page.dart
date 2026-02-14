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


class AccountPanel extends StatelessWidget {
  const AccountPanel({super.key});

  @override
  Widget build(BuildContext context) {
    final accounts = context.watch<AccountStore>().accounts;
    final accountStore = context.read<AccountStore>();
    return Column(
      spacing: 20,
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
        SizedBox(
          height: 144,
          child: ListView.separated(
            scrollDirection: Axis.horizontal,
              itemCount: accounts.length,
              itemBuilder: (context,index) => AccountCard(accounts[index]),
              separatorBuilder: (context,index) => SizedBox(width: 12,),
          ),
        ),
      ],
    );
  }
}

class AccountCard extends StatelessWidget {
  final Account account;
  const AccountCard(this.account,{super.key});

  @override
  Widget build(BuildContext context) {
    // FIXME: make it fill with a max with
    return Container(
      padding: EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(12),
        boxShadow: Style.shadowPurpleMd,
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [TextLabel(account.name), TextLabel("${account.startingBalance}")],
      ),
    );
  }
}
