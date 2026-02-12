import 'package:flutter/material.dart' hide IconButton, Colors;
import 'package:folio/components.dart';
import 'package:folio/state.dart';
import 'package:folio/colors.dart';
import 'package:lucide_icons_flutter/lucide_icons.dart';
import 'package:provider/provider.dart';
import 'categories.dart';
import 'general.dart';
import 'income_streams.dart';

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
