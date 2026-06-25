#pragma once

#include <QAbstractTableModel>
#include <string>
#include <QQmlEngine>
#include <span>

namespace folio {
    struct Account {
        std::string id;
        std::string name;
        float startingBalance;
    };

    class AccountModel : public QAbstractListModel {
        Q_OBJECT
        QML_ELEMENT

        std::vector<Account> accounts;
    public:

        Q_INVOKABLE void addAccount(QString name,float startingBalance);

        void loadAccounts(std::span<Account> accounts);

        // Returns the account with the provided id, if it exists.
        std::optional<Account> getAccount(std::string_view id) const;

        int rowCount(const QModelIndex &index = QModelIndex()) const override;

        QVariant data(const QModelIndex &index, int role) const override;

        QHash<int, QByteArray> roleNames() const override {
            return { {Qt::DisplayRole, "display"} };
        }
    };

}