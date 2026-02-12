import 'package:flutter/material.dart' hide IconButton, Colors,EditableText;
import 'package:folio/components.dart';
import 'package:folio/state.dart';
import 'package:lucide_icons_flutter/lucide_icons.dart';
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
            IconButton(icon: LucideIcons.plus,onTap: () => settings.addCategory(title: "New category"),),
          ],
        ),
        Expanded(
          // TODO: try adding keys for better performance
          child: ListView.separated(
            itemCount: categories.length,
            itemBuilder: (context, index) => CategoryCard(categories[index]),
            separatorBuilder: (BuildContext context, int index) => SizedBox(height: 16,),
          ),
        ),
      ],
    );
  }
}

// Optional: Unfocus the field to hide the keyboard
// FocusScope.of(context).unfocus();
class CategoryCard extends StatelessWidget {
  final TextEditingController? _controller;
  final Category category;
  CategoryCard(this.category,{super.key}): _controller = TextEditingController(text: category.title);

  @override
  Widget build(BuildContext context) {
    final settings = context.read<SettingsStore>();
    return Row(
      children: [
        Expanded(child: InlineTextField(
          controller: TextEditingController(text: category.title),

        )),
        Spacer(),
        IconButton(
          icon: LucideIcons.trash2,
          onTap: () => settings.deleteCategory(category.id),
        ),
      ],
    );
  }
}

