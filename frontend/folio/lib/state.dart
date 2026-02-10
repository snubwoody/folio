import 'package:flutter/material.dart';
import 'package:folio/database.dart';

class Category {
  final String id;
  final String title;

  const Category({this.id = "", this.title = ""});
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
    // _categories.add(category);
    _db.addCategory(title: title);
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

class AccountStore extends ChangeNotifier {
  final List<Account> _accounts = [
    Account(name: "Account 1", balance: 24, id: "1"),
    Account(name: "Account 2", balance: 24, id: "2"),
    Account(name: "Account 3", balance: 24, id: "3"),
  ];

  List<Account> get accounts => _accounts;

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
