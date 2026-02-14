import 'package:flutter/material.dart' hide Colors, IconButton;
import 'package:folio/colors.dart';
import 'package:folio/database.dart';
import 'package:folio/home_page.dart';
import 'package:folio/components.dart';
import 'package:folio/settings/settings_panel.dart';
import 'package:folio/state.dart';
import 'package:logging/logging.dart';
import 'package:lucide_icons_flutter/lucide_icons.dart';
import 'package:provider/provider.dart';

final log = Logger("Folio");


void main() {
  // TODO: try to get line number and file
  Logger.root.onRecord.listen((record) {
    print('${record.level.name}: ${record.time}: ${record.message}');
  });
  final database = AppDatabase(openConnection());
  log.info("Loading app");
  runApp(MyApp(database));
}

class MyApp extends StatelessWidget {
  final AppDatabase _db;
  MyApp(this._db, {super.key});
  @override
  Widget build(BuildContext context) {
    // TODO: pass this in the constructor?
    return MultiProvider(
      providers: [
        ChangeNotifierProvider(
          create: (BuildContext context) {
            final store = AccountStore(_db);
            store.load();
            log.info("Initialised account store");
            return store;
          },
        ),
        ChangeNotifierProvider(
          create: (BuildContext context) {
            final store = SettingsStore(_db);
            store.load();
            log.info("Initialised settings store");
            return store;
          },
        ),
        ChangeNotifierProvider(
          create: (BuildContext context) {
            final store = TransactionsStore(_db);
            store.load();
            log.info("Initialised transaction store");
            return store;
          },
        ),
      ],
      child: MaterialApp(
        debugShowCheckedModeBanner: false,
        title: 'Folio',
        theme: ThemeData(
          colorScheme: ColorScheme(
            brightness: Brightness.light,
            primary: Colors.purple500,
            onPrimary: Colors.white,
            secondary: Colors.purple500,
            onSecondary: Colors.white,
            error: Colors.purple500,
            onError: Colors.white,
            surface: Colors.white,
            onSurface: Colors.black,
          ),
        ),
        home: Scaffold(
          body: Row(
            children: const [
              NavigationPanel(),
              Expanded(
                child: Padding(
                  padding: EdgeInsets.all(24.0),
                  child: HomePage(),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class NavigationPanel extends StatelessWidget {
  const NavigationPanel({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: EdgeInsets.symmetric(vertical: 24, horizontal: 12),
      color: Colors.neutral25,
      child: Column(
        spacing: 24,
        children: [
          const IconButton(icon: LucideIcons.house),
          const IconButton(icon: LucideIcons.chartArea),
          const Spacer(),
          IconButton(
            icon: LucideIcons.settings,
            onTap: () => showDialog(
              context: context,
              builder: (BuildContext context) => SettingsPanel(),
            ),
          ),
        ],
      ),
    );
  }
}
