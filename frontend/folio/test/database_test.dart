import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:folio/database.dart';
import 'package:folio/dirs.dart';
import 'package:folio/main.dart';
import 'package:path_provider/path_provider.dart';

void main() {
  group("AppDatabase", (){
    test(".addCategory() adds a category with the title", () async {
      final database = AppDatabase(openConnection());
      final id = await database.addCategory(title: "New item");
      final items = await (database.select(database.categories)..where((r) => r.id.equals(id))).get();
      assert(items[0].title == "New item");

    });
  });

  test("Get temp directory", () async {
    final dir = getTempDirectory();
    assert(dir.path == Directory.systemTemp.path);
  });
}
