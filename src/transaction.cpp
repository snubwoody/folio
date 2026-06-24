#include "transaction.h"


QVariant TransactionTableModel::data(const QModelIndex &index, int role) const {
    if (role != Qt::DisplayRole){
        return QVariant();
    }

    const auto transaction = transactions[index.row()];

    switch (index.column()) {
        case 0:
            return QString::fromStdString(transaction.date);
        case 1:
            return QString("Account");
        case 2:
            return QString("Payee");
        case 3:
            return QString("Note");
        case 4:
            return QString("Category");
        case 5:
            return QString("Outflow");
        case 6:
            return QString("Inflow");
        default:
            break;
    }
    return QString("%1, %2").arg(index.column()).arg(index.row());
}

void TransactionTableModel::loadTransactions(const std::vector<Transaction> &transactions){
    for (const auto &transaction : transactions) {
        this->transactions.push_back(transaction);
    }
}

int TransactionTableModel::rowCount(const QModelIndex &index ) const {
    return transactions.capacity();
}

int TransactionTableModel::columnCount(const QModelIndex &index ) const {
    return 7;
}
