import 'package:flutter/material.dart';
import 'package:folio/database.dart';

class Category {
  final String id;
  final String title;

  const Category({this.id = "", this.title = ""});
}

class Expense {
  final String id;
  final Account? account;
  final Category? category;
  final DateTime date;
  final int amount;

  Expense({
    this.id = "",
    this.account,
    this.category,
    this.amount = 0,
    DateTime? date
  }): date = date ?? DateTime.now().toUtc();
}

class Income {
  final String id;
  final Account? account;
  final IncomeStream? incomeStream;
  final DateTime date;
  final int amount;

  Income({
    this.id = "",
    this.account,
    this.incomeStream,
    this.amount = 0,
    DateTime? date
  }): date = date ?? DateTime.now().toUtc();
}

class IncomeStream {
  final String id;
  final String title;

  const IncomeStream({this.id = "", this.title = ""});
}

class Account {
  final String id;
  final String name;
  final double balance;
  final double startingBalance;

  const Account({
    this.name = "",
    this.id = "",
    this.balance = 0,
    this.startingBalance = 0,
  });
}


class SettingsStore extends ChangeNotifier {
  SettingsStore(this._db);

  final AppDatabase _db;
  List<Category> _categories = [];
  List<IncomeStream> _incomeStreams = [];

  /// A list of all the categories.
  List<Category> get categories => _categories;

  /// A list of the all income streams.
  List<IncomeStream> get incomeStreams => _incomeStreams;

  /// Load the data from the database
  Future<void> load() async {
    _categories = await _db.getCategories();
    _incomeStreams = await _db.getIncomeStreams();
    notifyListeners();
  }

  /// Add a new category to the categories list.
  Future<void> addCategory({required String title}) async {
    _db.addCategory(title: title);
    await load();
    notifyListeners();
  }

  /// Edit a category
  Future<void> editCategory({required String id, required String title}) async {
    _db.editCategory(id: id, title: title);
    await load();
    notifyListeners();
  }

  /// Edit an income stream
  Future<void> editIncomeStream({required String id, required String title}) async {
    _db.editIncomeStream(id: id, title: title);
    await load();
    notifyListeners();
  }

  /// Add a new income stream to the list.
  void addIncomeStream(IncomeStream stream) {
    _incomeStreams.add(stream);
    notifyListeners();
  }

  /// Deletes a category.
  Future<void> deleteCategory(String id) async {
    await _db.deleteCategory(id);
    await load();
    notifyListeners();
  }

  void deleteIncomeStream(String id) {
    // TODO: get the first one just in case things have duplicate ids?
    _incomeStreams.retainWhere((i) => i.id != id);
    notifyListeners();
  }
}


class TransactionsStore extends ChangeNotifier {
  TransactionsStore(this._db);

  final AppDatabase _db;
  List<Expense> _expenses = [
    Expense(id: "",account: Account(name: "RBC"),category: Category(title: "Groceries"),amount: 24),
    Expense(id: "",account: Account(name: "RBC"),category: Category(title: "Groceries"),amount: 24),
    Expense(id: "",account: Account(name: "RBC"),category: Category(title: "Groceries"),amount: 24),
    Expense(id: "",account: Account(name: "RBC"),category: Category(title: "Groceries"),amount: 24),
  ];

  List<Income> _incomes = [];

  /// A list of all the expenses.
  List<Expense> get expenses => _expenses;
  List<Income> get incomes => _incomes;

  /// Load the data from the database
  Future<void> load() async {
    // _categories = await _db.getCategories();
    // _incomeStreams = await _db.getIncomeStreams();
    notifyListeners();
  }
}

class AccountStore extends ChangeNotifier {
  final AppDatabase _db;
  AccountStore(this._db);

  List<Account> _accounts = [];

  List<Account> get accounts => _accounts;

  /// Load the data from the database
  Future<void> load() async {
    _accounts = await _db.getAccounts();
    notifyListeners();
  }

  void addAccount(Account account) {
    _accounts.add(account);
    notifyListeners();
  }

  void deleteAccount(String id) {
    // TODO: get the first one just in case things have duplicate ids?
    _accounts.retainWhere((account) => account.id != id);
    notifyListeners();
  }
}
