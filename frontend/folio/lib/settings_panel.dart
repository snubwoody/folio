import 'package:flutter/material.dart' hide IconButton;
import 'package:folio/components.dart';
import 'package:lucide_icons_flutter/lucide_icons.dart';

// TODO: generator function for currencies

class SettingsPanel extends StatelessWidget {
  const SettingsPanel({super.key});

  @override
  Widget build(BuildContext context) {
    return const Dialog(
      child: Row(
        children: [
          Column(
            spacing: 12,
            children: [
              TextLabel("General"),
              TextLabel("Categories"),
              TextLabel("Income streams"),
            ],
          ),
          Expanded(child: GeneralSettings()),
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
      children: [
        Row(
          children: [
            Column(
              children: [
                TextLabel("Currency code"),
                TextLabel("The ISO currency code"),
              ],
            ),
            Spacer(),
            TextLabel("CAD"),
          ],
        ),
        Column(
          children: [
            TextLabel("Accounts")
          ],
        )
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
