#pragma once

#include <QAbstractTableModel>
#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <string>
#include <vector>

#include "category.h"

namespace folio {

    struct Transaction {
        std::string id;
        std::string date;
        std::optional<std::string> categoryId;
        float amount;
    };

    class TransactionTableModel : public QAbstractTableModel {
        Q_OBJECT
        QML_ELEMENT

        std::vector<Transaction> transactions;
        CategoryModel* categoryModel;

      public:
        TransactionTableModel(CategoryModel* categoryModel) : categoryModel(categoryModel) {}

        void loadTransactions(std::span<Transaction> transactions);

        int rowCount(const QModelIndex& index = QModelIndex()) const override;

        int columnCount(const QModelIndex& index = QModelIndex()) const override;

        QVariant data(const QModelIndex& index, int role) const override;

        QVariant headerData(int section, Qt::Orientation orientation, int role) const override;

        QHash<int, QByteArray> roleNames() const override {
            return {{Qt::DisplayRole, "display"}};
        }
    };

} // namespace folio
