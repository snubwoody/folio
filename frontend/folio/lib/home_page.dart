import 'package:flutter/material.dart' hide Colors;
import 'package:folio/colors.dart';
import 'package:folio/components.dart';
import 'package:folio/style.dart';


class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(24.0),
      child: Column(children: [
        AccountPanel(),
      ],),
    );
  }
}

class AccountPanel extends StatelessWidget {
  const AccountPanel({super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Row(
          children: [
            TextLabel("Accounts"),
            Spacer(),
            TextLabel("New")
          ],
        ),
        Row(
          spacing: 24,
          children: [
            Account(name: "RBC",balance: 24,),
            Account(name: "BOC",balance: 224,),
            Account(),
            Account(),
          ],
        )
      ],
    );
  }
}


class Account extends StatelessWidget {
  final String name;
  final double balance;
  const Account({super.key,this.name = "",this.balance = 0});

  @override
  Widget build(BuildContext context) {
    // FIXME: make it fill with a max with
    return  ConstrainedBox(
      constraints: BoxConstraints(maxWidth: double.infinity),
      child: Container(
        padding: EdgeInsets.all(16),
        decoration: BoxDecoration(
          color: Colors.white,
          borderRadius: BorderRadius.circular(12),
          boxShadow: Style.shadowPurpleMd
        ),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            TextLabel(name),
            TextLabel("$balance"),
          ],
        ),
      ),
    );
  }
}





