import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:folio/database.dart';
import 'package:folio/dirs.dart';

void main() {
  final database = AppDatabase(openConnection());
  group("AppDatabase", () {
    test(".addCategory() adds a category with the title", () async {
      final id = await database.addCategory(title: "New item");
      final items = await (database.select(
        database.categories,
      )..where((r) => r.id.equals(id))).get();
      assert(items[0].title == "New item");
    });
    test(".addIncomeStream() adds an income stream with the title", () async {
      final id = await database.addIncomeStream(title: "Stream 1");
      final items = await (database.select(
        database.incomeStreams,
      )..where((r) => r.id.equals(id))).get();
      assert(items[0].title == "Stream 1");
    });
    test(".addAccount() adds an account with the properties", () async {
      final id = await database.addAccount(
        name: "Account 24",
        startingBalance: 200,
      );
      final items = await (database.select(
        database.accounts,
      )..where((r) => r.id.equals(id))).get();
      assert(items[0].name == "Account 24");
      assert(items[0].startingBalance == 200);
    });
    test(".getCategories() returns all the categories", () async {
      await database.addCategory(title: "New category");
      final categories = await database.getCategories();
      final rows = await database.select(database.categories).get();
      assert(categories.length == rows.length);
      assert(categories[0].title == categories[0].title);
      assert(categories[0].id == categories[0].id);
    });
    test(".getIncomeStreams() returns all the income streams", () async {
      await database.addIncomeStream(title: "New stream");
      final streams = await database.getIncomeStreams();
      final rows = await database.select(database.incomeStreams).get();
      assert(streams.length == rows.length);
      assert(streams[0].title == streams[0].title);
      assert(streams[0].id == streams[0].id);
    });
    test(".editCategory() updates the title", () async {
      final id = await database.addCategory(title: "Category 1");
      await database.editCategory(title: "Category 2", id: id);
      final category = await (database.select(
        database.categories,
      )..where((r) => r.id.equals(id))).getSingle();
      assert(category.title == "Category 2");
    });
    test(".editIncomeStream() updates the title", () async {
      final id = await database.addIncomeStream(title: "IncomeStream 1");
      await database.editIncomeStream(title: "IncomeStream 2", id: id);
      final stream = await (database.select(
        database.incomeStreams,
      )..where((r) => r.id.equals(id))).getSingle();
      assert(stream.title == "IncomeStream 2");
    });
  });

  test("Get temp directory", () async {
    final dir = getTempDirectory();
    assert(dir.path == Directory.systemTemp.path);
  });
}
