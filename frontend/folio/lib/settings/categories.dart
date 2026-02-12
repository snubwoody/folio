import 'package:flutter/material.dart' hide IconButton, Colors, EditableText;
import 'package:folio/components.dart';
import 'package:folio/state.dart';
import 'package:lucide_icons_flutter/lucide_icons.dart';
import 'package:path_provider/path_provider.dart';
import 'package:provider/provider.dart';

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
            IconButton(
              icon: LucideIcons.plus,
              onTap: () => settings.addCategory(title: "New category"),
            ),
          ],
        ),
        Expanded(
          // TODO: try adding keys for better performance
          child: ListView.separated(
            itemCount: categories.length,
            itemBuilder: (context, index) => CategoryCard(categories[index]),
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
class CategoryCard extends StatelessWidget {
  final TextEditingController? _controller;
  final Category category;
  CategoryCard(this.category, {super.key})
    : _controller = TextEditingController(text: category.title);

  Future<void> editCategory(BuildContext context, String value) async {
    final store = context.read<SettingsStore>();
    await store.editCategory(id: category.id, title: value);
  }

  @override
  Widget build(BuildContext context) {
    final settings = context.read<SettingsStore>();
    return Row(
      children: [
        Expanded(
          child: InlineTextField(
            controller: _controller,
            onSubmitted: (String value) => editCategory(context, value),
          ),
        ),
        Spacer(),
        IconButton(
          icon: LucideIcons.trash2,
          onTap: () => settings.deleteCategory(category.id),
        ),
      ],
    );
  }
}
