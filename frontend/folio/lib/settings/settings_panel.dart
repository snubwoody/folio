import 'package:flutter/material.dart' hide IconButton, Colors;
import 'package:folio/components.dart';
import 'package:folio/state.dart';
import 'package:folio/colors.dart';
import 'package:lucide_icons_flutter/lucide_icons.dart';
import 'package:provider/provider.dart';

// TODO: generator function for currencies

class SettingsPanel extends StatefulWidget {
  const SettingsPanel({super.key});

  @override
  State<SettingsPanel> createState() => _SettingsPanelState();
}

class _SettingsPanelState extends State<SettingsPanel> {
  int _index = 0;

  setIndex(int index) {
    setState(() {
      _index = index;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Dialog(
      child: Row(
        children: [
          Container(
            color: Colors.neutral25,
            padding: EdgeInsets.symmetric(vertical: 24, horizontal: 12),
            child: Column(
              spacing: 12,
              children: [
                PanelButton(
                  onTap: () => setIndex(0),
                  text: "General",
                  selected: _index == 0,
                ),
                PanelButton(
                  onTap: () => setIndex(1),
                  text: "Categories",
                  selected: _index == 1,
                ),
                PanelButton(
                  onTap: () => setIndex(2),
                  text: "Income streams",
                  selected: _index == 2,
                ),
              ],
            ),
          ),
          Expanded(
            child: Padding(
              padding: EdgeInsets.all(24.0),
              child: switch (_index) {
                // TODO: Handle this case.
                1 => CategoriesSection(),
                2 => IncomeStreamsSection(),
                _ => GeneralSection(),
              },
            ),
          ),
        ],
      ),
    );
  }
}

class CategoriesSection extends StatelessWidget {
  const CategoriesSection({super.key});

  @override
  Widget build(BuildContext context) {
    final categories = context.watch<SettingsStore>().categories;
    final settings = context.read<SettingsStore>();
    return Column(
      spacing: 16,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          children: [
            Column(
              spacing: 4,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                TextLabel("Categories"),
                TextLabel("Categories are used for organising expenses."),
              ],
            ),
            Spacer(),
            IconButton(icon: LucideIcons.plus,onTap: () => settings.addCategory(title: "New category"),),
          ],
        ),
        Expanded(
          child: ListView.builder(
            itemCount: categories.length,
            itemBuilder: (context, index) {
              final category = categories[index];
              return Row(
                children: [
                  TextLabel(category.title),
                  Spacer(),
                  IconButton(
                    icon: LucideIcons.trash2,
                    onTap: () => settings.deleteCategory(category.id),
                  ),
                ],
              );
            },
          ),
        ),
        // ...categories.map(
        //   (c) => Row(
        //     children: [
        //       TextLabel(c.title),
        //       Spacer(),
        //       IconButton(
        //         icon: LucideIcons.trash2,
        //         onTap: () => settings.deleteCategory(c.id),
        //       ),
        //     ],
        //   ),
        // ),
      ],
    );
  }
}

class IncomeStreamsSection extends StatelessWidget {
  const IncomeStreamsSection({super.key});

  @override
  Widget build(BuildContext context) {
    return TextLabel("Hi");
  }
}

class GeneralSection extends StatelessWidget {
  const GeneralSection({super.key});

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
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      spacing: 16,
      children: [
        TextLabel("Accounts"),
        ...accounts.map(
          (account) => Row(
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
              IconButton(
                icon: LucideIcons.trash2,
                onTap: () => accountStore.deleteAccount(account.id),
              ),
            ],
          ),
        ),
      ],
    );
  }
}

class PanelButton extends StatelessWidget {
  final void Function() onTap;
  final String text;
  final bool selected;
  const PanelButton({
    super.key,
    required this.onTap,
    required this.text,
    required this.selected,
  });

  @override
  Widget build(BuildContext context) {
    return BaseButton(
      color: selected ? Colors.purple500 : null,
      hoverColor: selected ? null : Colors.neutral50,
      onTap: onTap,
      borderRadius: BorderRadius.circular(8),
      padding: EdgeInsets.symmetric(vertical: 4, horizontal: 12),
      child: TextLabel(
        text,
        color: selected ? Colors.neutral25 : Colors.neutral950,
      ),
    );
  }
}
