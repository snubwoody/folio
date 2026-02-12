import 'package:flutter/material.dart' hide IconButton, Colors, EditableText;
import 'package:folio/components.dart';
import 'package:folio/state.dart';
import 'package:lucide_icons_flutter/lucide_icons.dart';
import 'package:provider/provider.dart';

class IncomeStreamsSection extends StatelessWidget {
  const IncomeStreamsSection({super.key});

  @override
  Widget build(BuildContext context) {
    final incomeStreams = context.watch<SettingsStore>().incomeStreams;
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
                TextLabel("Income streams"),
                TextLabel("Income streams are used for organising incomes."),
              ],
            ),
            Spacer(),
            IconButton(
              icon: LucideIcons.plus,
              onTap: () => settings.addCategory(title: "New category"),
            ),
          ],
        ),
        Expanded(
          // TODO: try adding keys for better performance
          child: ListView.separated(
            itemCount: incomeStreams.length,
            itemBuilder: (context, index) => IncomeStreamCard(incomeStreams[index]),
            separatorBuilder: (BuildContext context, int index) =>
                SizedBox(height: 16),
          ),
        ),
      ],
    );
  }
}

// TODO: Unfocus the field to hide the keyboard
// FocusScope.of(context).unfocus();
// TODO: edit on focus lost
// TODO: dispose controller
class IncomeStreamCard extends StatelessWidget {
  final TextEditingController? _controller;
  final IncomeStream incomeStream;
  IncomeStreamCard(this.incomeStream, {super.key})
      : _controller = TextEditingController(text: incomeStream.title);

  Future<void> editIncomeStream(BuildContext context, String value) async {
    final store = context.read<SettingsStore>();
    await store.editIncomeStream(id: incomeStream.id, title: value);
  }

  @override
  Widget build(BuildContext context) {
    final settings = context.read<SettingsStore>();
    return Row(
      children: [
        Expanded(
          child: InlineTextField(
            controller: _controller,
            onSubmitted: (String value) => editIncomeStream(context, value),
          ),
        ),
        Spacer(),
        IconButton(
          icon: LucideIcons.trash2,
          onTap: () => settings.deleteCategory(incomeStream.id),
        ),
      ],
    );
  }
}
