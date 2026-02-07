import 'package:flutter/material.dart';

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
