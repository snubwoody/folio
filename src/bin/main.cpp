#include <ostream>
#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <string>
#include <qqml.h>
#include <QQmlContext>
#include <vector>
#include "../transaction.h"
#include "../category.h"
#include <print>

#include "../account.h"

using namespace folio;

int main(int argc, char *argv[]) {
    QCoreApplication::setApplicationName("Folio");
    QCoreApplication::setApplicationVersion("3.0.0");
    QGuiApplication app(argc, argv);

    AccountModel accountModel{};
    CategoryModel categoryModel{};
    TransactionTableModel transactionModel{&categoryModel};

    std::vector<Transaction> transactions{
        Transaction{
            .id = "T1",
            .date = "01/01/2026",
            .amount = 200,
        },
        Transaction{
            .id = "T2",
            .date = "01/01/2026",
            .categoryId = "C2",
            .amount = 22400,
        },
        Transaction{
            .id = "T3",
            .date = "01/01/2026",
            .categoryId = "C1",
            .amount = 2002,
        },
    };
    transactionModel.loadTransactions(transactions);

    std::vector<Category> categories{
        Category{
            .id = "C1",
            .title = "Groceries"
        },
        Category{
            .id = "C2",
            .title = "Transport"
        },
        Category{
            .id = "C3",
            .title = "Rent"
        },
        Category{
            .id = "C4",
            .title = "Entertainment"
        },
    };
    categoryModel.loadCategories(categories);

    std::vector<Account>  accounts{
        Account{
            .id = "A1",
            .name = "RBC Credit Card"
        },
        Account{
            .id = "A2",
            .name = "Absa Chequing"
        },
        Account{
            .id = "A3",
            .name = "FNB Savings"
        },
    };
    accountModel.loadAccounts(accounts);

    QQmlApplicationEngine engine;
    engine.rootContext()->setContextProperty("transactionTableModel",&transactionModel);
    engine.rootContext()->setContextProperty("categoryModel",&categoryModel);
    engine.rootContext()->setContextProperty("accountModel",&accountModel);
    engine.load(QUrl(QStringLiteral("qrc:/Main.qml")));
    if (engine.rootObjects().isEmpty())
        return -1;

    return app.exec();
}
