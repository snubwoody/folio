import 'dart:io';
import 'dart:math';
import 'package:drift/drift.dart';
import 'package:drift/native.dart';
import 'package:drift_flutter/drift_flutter.dart';
import 'package:folio/state.dart';
import 'package:path_provider/path_provider.dart';

part 'database.g.dart';

// TODO: 6 digits after decimal
@DataClassName("CategoryRow")
class Categories extends Table {
  TextColumn get id => text()();
  TextColumn get title => text()();
  IntColumn get createdAt => integer()();

  @override
  Set<Column<Object>> get primaryKey => {id};
}

@DataClassName("IncomeStreamRow")
class IncomeStreams extends Table {
  TextColumn get id => text()();
  TextColumn get title => text()();
  IntColumn get createdAt => integer()();

  @override
  Set<Column<Object>> get primaryKey => {id};
}

// TODO: on delete
@DataClassName("BudgetRow")
class Budgets extends Table {
  TextColumn get id => text()();
  IntColumn get amount => integer()();
  IntColumn get createdAt => integer()();
  TextColumn get categoryId => text().references(Categories, #id)();

  @override
  Set<Column<Object>> get primaryKey => {id};
}

@DataClassName("AccountRow")
class Accounts extends Table {
  TextColumn get id => text()();
  IntColumn get startingBalance => integer()();
  TextColumn get name => text()();
  IntColumn get createdAt => integer()();

  @override
  Set<Column<Object>> get primaryKey => {id};
}

// TODO dates
@DataClassName("ExpenseRow")
class Expenses extends Table {
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

@DataClassName("IncomeRow")
class Incomes extends Table {
  TextColumn get id => text()();
  IntColumn get amount => integer()();
  DateTimeColumn get transactionDate => dateTime()();
  TextColumn get incomeStreamId =>
      text().nullable().references(IncomeStreams, #id)();
  TextColumn get accountId => text().nullable().references(Accounts, #id)();
  IntColumn get createdAt => integer()();
  // TODO: Maybe deprecate
  TextColumn get currencyCode => text()();

  @override
  Set<Column<Object>> get primaryKey => {id};
}

@DriftDatabase(
  tables: [Categories, Incomes, Expenses, Accounts, IncomeStreams, Budgets],
)
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

  /// Fetches all the categories from the database
  Future<List<Category>> getCategories() async {
    final rows = await select(categories).get();
    return rows.map((r) => Category(id: r.id, title: r.title)).toList();
  }

  /// Fetches all the income streams from the database
  Future<List<IncomeStream>> getIncomeStreams() async {
    final rows = await select(incomeStreams).get();
    return rows.map((r) => IncomeStream(id: r.id, title: r.title)).toList();
  }

  // TODO: test these
  /// Adds a new category to the database and returns the `id`.
  Future<String> addCategory({required String title, String? id}) async {
    final rowId = id ?? generateRandomString(20);
    final int epochSeconds = (DateTime.now().millisecondsSinceEpoch / 1000)
        .toInt();
    await into(categories).insert(
      CategoriesCompanion.insert(
        title: title,
        id: rowId,
        createdAt: epochSeconds,
      ),
    );
    return rowId;
  }

  Future<void> editCategory({required String title, required String id}) async {
    await (update(categories)..where((r) => r.id.equals(id))).write(
      CategoriesCompanion(title: Value(title)),
    );
  }

  Future<void> editIncomeStream({
    required String title,
    required String id,
  }) async {
    await (update(incomeStreams)..where((r) => r.id.equals(id))).write(
      IncomeStreamsCompanion(title: Value(title)),
    );
  }

  // TODO: test these
  /// Deletes a row from the `categories` table with the matching `id`.
  Future<void> deleteCategory(String id) async {
    await (delete(categories)..where((r) => r.id.equals(id))).go();
  }

  /// Deletes a row from the `incomeStreams` table with the matching `id`.
  Future<void> deleteIncomeStream(String id) async {
    await (delete(incomeStreams)..where((r) => r.id.equals(id))).go();
  }

  /// Adds a new income stream to the database and returns the `id`.
  Future<String> addIncomeStream({required String title, String? id}) async {
    final rowId = id ?? generateRandomString(20);
    final int epochSeconds = (DateTime.now().millisecondsSinceEpoch / 1000)
        .toInt();
    await into(incomeStreams).insert(
      IncomeStreamsCompanion.insert(
        title: title,
        id: rowId,
        createdAt: epochSeconds,
      ),
    );
    return rowId;
  }

  /// Adds a new accounts to the database and returns the `id`.
  Future<String> addAccount({
    required String name,
    int startingBalance = 0,
    String? id,
  }) async {
    // TODO: 6 digits after the decimal
    final rowId = id ?? generateRandomString(20);
    final int epochSeconds = (DateTime.now().millisecondsSinceEpoch / 1000)
        .toInt();
    await into(accounts).insert(
      AccountsCompanion.insert(
        startingBalance: startingBalance,
        name: name,
        id: rowId,
        createdAt: epochSeconds,
      ),
    );
    return rowId;
  }
}

/// Opens a database connection.
LazyDatabase openConnection({String path = "data.sqlite"}) {
  File(path).createSync(recursive: true);
  final file = File(path);
  return LazyDatabase(() async {
    return NativeDatabase.createInBackground(file);
  }, openImmediately: true);
}

/// Generates a random alphanumeric string
String generateRandomString(int len) {
  var rng = Random();
  const chars =
      'AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz1234567890';
  return List.generate(len, (index) => chars[rng.nextInt(chars.length)]).join();
}
