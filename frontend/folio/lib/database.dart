import 'dart:io';
import 'dart:math';
import 'package:drift/drift.dart';
import 'package:drift/native.dart';
import 'package:drift_flutter/drift_flutter.dart';
import 'package:folio/settings_panel.dart';
import 'package:path_provider/path_provider.dart';

part 'database.g.dart';

class TodoItems extends Table {
  IntColumn get id => integer().autoIncrement()();
  TextColumn get title => text().withLength(min: 6, max: 32)();
  TextColumn get content => text().named('body')();
  DateTimeColumn get createdAt => dateTime().nullable()();
}

// TODO: 6 digits after decimal
class Categories extends Table{
  TextColumn get id => text()();
  TextColumn get title => text()();
  IntColumn get createdAt => integer()();

  @override
  Set<Column<Object>> get primaryKey => {id};
}

class IncomeStreams extends Table{
  TextColumn get id => text()();
  TextColumn get title => text()();
  IntColumn get createdAt => integer()();

  @override
  Set<Column<Object>> get primaryKey => {id};
}

// TODO: on delete
class Budgets extends Table{
  TextColumn get id => text()();
  IntColumn get amount => integer()();
  IntColumn get createdAt => integer()();
  TextColumn get categoryId => text().references(Categories, #id)();

  @override
  Set<Column<Object>> get primaryKey => {id};
}

class Accounts extends Table{
  TextColumn get id => text()();
  IntColumn get startingBalance => integer()();
  TextColumn get name => text()();
  IntColumn get createdAt => integer()();

  @override
  Set<Column<Object>> get primaryKey => {id};
}

// TODO dates
class Expenses extends Table{
  TextColumn get id => text()();
  IntColumn get amount => integer()();
  DateTimeColumn get transactionDate => dateTime()();
  TextColumn get categoryId => text().nullable().references(Categories, #id)();
  TextColumn get accountId => text().nullable().references(Accounts, #id)();
  IntColumn get createdAt => integer()();
  // TODO: Maybe deprecate
  TextColumn get currencyCode => text()();

  @override
  Set<Column<Object>> get primaryKey => {id};
}

class Incomes extends Table{
  TextColumn get id => text()();
  IntColumn get amount => integer()();
  DateTimeColumn get transactionDate => dateTime()();
  TextColumn get incomeStreamId => text().nullable().references(IncomeStreams, #id)();
  TextColumn get accountId => text().nullable().references(Accounts, #id)();
  IntColumn get createdAt => integer()();
  // TODO: Maybe deprecate
  TextColumn get currencyCode => text()();

  @override
  Set<Column<Object>> get primaryKey => {id};
}

@DriftDatabase(tables: [Categories,Incomes,Expenses,Accounts,IncomeStreams,Budgets])
class AppDatabase extends _$AppDatabase {
  // After generating code, this class needs to define a `schemaVersion` getter
  // and a constructor telling drift where the database should be stored.
  // These are described in the getting started guide: https://drift.simonbinder.eu/setup/
  AppDatabase([QueryExecutor? executor]) : super(executor ?? _openConnection());

  @override
  int get schemaVersion => 1;

  static QueryExecutor _openConnection() {
    return driftDatabase(
      name: 'my_database',
      native: const DriftNativeOptions(
        // By default, `driftDatabase` from `package:drift_flutter` stores the
        // database files in `getApplicationDocumentsDirectory()`.
        databaseDirectory: getApplicationSupportDirectory,
      ),
      // If you need web support, see https://drift.simonbinder.eu/platforms/web/
    );
  }

  // TODO: test these
  /// Adds a new category to the database and returns the `id`.
  Future<String> addCategory({
    required String title,
    String? id,
  })async {
    final rowId = id ?? generateRandomString(20);
    final int epochSeconds = (DateTime.now().millisecondsSinceEpoch / 1000).toInt() ;
    await into(categories)
        .insert(CategoriesCompanion.insert(title: title, id: rowId,createdAt: epochSeconds));
    return rowId;
  }
}

/// Opens a database connection.
LazyDatabase openConnection({String path = "data.sqlite"}){
  File(path).createSync(recursive: true);
  final file = File(path);
  return LazyDatabase(()async{
    return NativeDatabase.createInBackground(file);
  },openImmediately: true);
}

/// Generates a random alphanumeric string
String generateRandomString(int len) {
  var r = Random();
  const _chars = 'AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz1234567890';
  return List.generate(len, (index) => _chars[r.nextInt(_chars.length)]).join();
}
