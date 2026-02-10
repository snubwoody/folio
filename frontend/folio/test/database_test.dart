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
      final id = await database.addAccount(name: "Account 24", startingBalance: 200);
      final items = await (database.select(
        database.accounts,
      )..where((r) => r.id.equals(id))).get();
      assert(items[0].name == "Account 24");
      assert(items[0].startingBalance == 200);
    });
  });

  test("Get temp directory", () async {
    final dir = getTempDirectory();
    assert(dir.path == Directory.systemTemp.path);
  });
}
