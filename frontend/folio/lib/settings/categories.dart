import 'package:flutter/material.dart' hide IconButton, Colors;
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
          child: ListView.separated(
            itemCount: categories.length,
            itemBuilder: (context, index) => CategoryCard(categories[index]),
            separatorBuilder: (BuildContext context, int index) => SizedBox(height: 24,),
          ),
        ),
      ],
    );
  }
}

class CategoryCard extends StatelessWidget {
  final Category category;
  const CategoryCard(this.category,{super.key});

  @override
  Widget build(BuildContext context) {
    final settings = context.read<SettingsStore>();
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 24.0),
      child: Row(
        children: [
          TextLabel(category.title),
          Spacer(),
          IconButton(
            icon: LucideIcons.trash2,
            onTap: () => settings.deleteCategory(category.id),
          ),
        ],
      ),
    );;
  }
}

