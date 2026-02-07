import 'package:flutter/material.dart' hide IconButton,Colors;
import 'package:folio/components.dart';
import 'package:folio/state.dart';
import 'package:folio/colors.dart';
import 'package:lucide_icons_flutter/lucide_icons.dart';
import 'package:provider/provider.dart';

// TODO: generator function for currencies

class SettingsPanel extends StatelessWidget {
  const SettingsPanel({super.key});

  @override
  Widget build(BuildContext context) {
    return Dialog(
      child: Row(
        children: [
          Container(
            color: Colors.neutral25,
            padding: EdgeInsets.symmetric(vertical: 24,horizontal: 12),
            child: Column(
              spacing: 12,
              children: [
                TextLabel("General"),
                TextLabel("Categories"),
                TextLabel("Income streams"),
              ],
            ),
          ),
          Expanded(child: Padding(
            padding: EdgeInsets.all(24.0),
            child: GeneralSettings(),
          )),
        ],
      ),
    );
  }
}

class GeneralSettings extends StatelessWidget {
  const GeneralSettings({super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
      spacing: 16,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
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
        ),
        AccountSection(),
      ],
    );
  }
}

class AccountSection extends StatelessWidget {
  const AccountSection({super.key});

  @override
  Widget build(BuildContext context) {
    final accountStore = context.read<AccountStore>();
    final accounts = context.watch<AccountStore>().accounts;
    print(accounts);
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      spacing: 16,
      children: [
        TextLabel("Accounts"),
        ...accounts.map((account) => Row(
          children: [
            Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              spacing: 4,
              children: [
              TextLabel(account.name),
              TextLabel("Starting balance: ${account.startingBalance}"),
              ],
            ),
            Spacer(),
            IconButton(icon: LucideIcons.trash2,onTap:() => accountStore.deleteAccount(account.id),)
          ],
        ))
      ],
    );
  }
}


class PanelButton extends StatelessWidget {
  const PanelButton({super.key});

  @override
  Widget build(BuildContext context) {
    return const Placeholder();
  }
}
