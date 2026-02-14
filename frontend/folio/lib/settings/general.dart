import 'package:flutter/material.dart' hide IconButton, Colors;
import 'package:folio/components.dart';
import 'package:folio/state.dart';
import 'package:lucide_icons_flutter/lucide_icons.dart';
import 'package:provider/provider.dart';

class GeneralSection extends StatelessWidget {
  const GeneralSection({super.key});

  @override
  Widget build(BuildContext context) {
    return ListView.builder(
      itemCount: 2,
      itemBuilder: (context,index) => switch(index){
        0 => CurrencySection(),
        _ => AccountSection()
      },
    );
  }
}

class CurrencySection extends StatelessWidget {
  const CurrencySection({super.key});

  @override
  Widget build(BuildContext context) {
    return const Row(
      children: [
        Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            TextLabel("Currency code"),
            TextLabel("The ISO currency code"),
          ],
        ),
        Spacer(),
        TextLabel("CAD"),
      ],
    );
  }
}


class AccountSection extends StatelessWidget {
  const AccountSection({super.key});

  @override
  Widget build(BuildContext context) {
    final accounts = context.watch<AccountStore>().accounts;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      spacing: 16,
      children: [
        TextLabel("Accounts"),
        ...accounts.map((account) => AccountCard(account)),
      ],
    );
  }
}

class AccountCard extends StatelessWidget {
  final Account account;
  const AccountCard(this.account,{super.key});

  Future<void> editAccount(BuildContext context,String value)async{

  }

  @override
  Widget build(BuildContext context) {
    final accountStore = context.read<AccountStore>();
    return Row(
      children: [
        Expanded(
          child: InlineTextField(
            controller: TextEditingController(text: account.name),
            onSubmitted: (String value) => editAccount(context, value),
          ),
        ),
        Spacer(),
        IconButton(
          icon: LucideIcons.trash2,
          onTap: () => accountStore.deleteAccount(account.id),
        ),
      ],
    );
  }
}

