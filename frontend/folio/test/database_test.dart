import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:folio/database.dart';
import 'package:folio/dirs.dart';
import 'package:folio/main.dart';
import 'package:path_provider/path_provider.dart';

void main() {
  test("Get temp directory", () async {
    final dir = getTempDirectory();
    assert(dir.path == Directory.systemTemp.path);
  });
  test("Open connection", () async {
    final db = await openConnection();
  });
}
