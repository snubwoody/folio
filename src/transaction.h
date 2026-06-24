#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <string>
#include <QAbstractTableModel>
#include <vector>

struct Transaction {
    std::string id;
    std::string date;
    float amount;
};


class TransactionTableModel : public QAbstractTableModel
{
    Q_OBJECT
    QML_ELEMENT

    std::vector<Transaction> transactions;

public:
    void loadTransactions(const std::vector<Transaction> &transactions);

    int rowCount(const QModelIndex &index = QModelIndex()) const override;

    int columnCount(const QModelIndex &index = QModelIndex()) const override;

    QVariant data(const QModelIndex &index, int role) const override;

    QHash<int, QByteArray> roleNames() const override {
        return { {Qt::DisplayRole, "display"} };
    }
};
