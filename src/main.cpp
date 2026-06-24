#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <string>
#include <qqml.h>
#include <QQmlContext>
#include <vector>
#include "transaction.h"

int main(int argc, char *argv[]) {

    QCoreApplication::setApplicationName("Folio");
    QCoreApplication::setApplicationVersion("3.0.0");
    QGuiApplication app(argc, argv);

    TransactionTableModel transactionModel{};

    const std::vector<Transaction> transactions{
        Transaction{
            .id = "T1",
            .date = "01/01/2026",
            .amount = 200,
        },
        Transaction{
            .id = "T2",
            .date = "01/01/2026",
            .amount = 22400,
        },
        Transaction{
            .id = "T3",
            .date = "01/01/2026",
            .amount = 2002,
        },
    };
    transactionModel.loadTransactions(transactions);

    QQmlApplicationEngine engine;
    engine.rootContext()->setContextProperty("transactionTableModel",&transactionModel);
    engine.load(QUrl(QStringLiteral("qrc:/main.qml")));
    if (engine.rootObjects().isEmpty())
        return -1;

    return app.exec();
}
