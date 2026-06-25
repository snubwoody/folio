#include "account.h"

void AccountModel::addAccount(QString name,float startingBalance) {
    int nextRow = accounts.size();
    beginInsertRows(QModelIndex(),nextRow,nextRow);
    const Account account{
        .id = std::to_string(std::rand()),
        .name = name.toStdString(),
        .startingBalance = startingBalance
    };

    accounts.push_back(account);
    endInsertRows();
}


QVariant AccountModel::data(const QModelIndex &index, int role) const {
    if (role != Qt::DisplayRole){
        return QVariant();
    }

    const auto account = accounts[index.row()];

    return QString::fromStdString(account.name);
}

void AccountModel::loadAccounts(std::span<Account> categories){
    for (const auto &category : categories) {
        this->accounts.push_back(category);
    }
}

int AccountModel::rowCount(const QModelIndex &index ) const {
    return accounts.size();
}

std::optional<Account> AccountModel::getAccount(std::string_view id) const{
    for (const auto &account: accounts) {
        if (account.id == id){
            return account;
        }
    }

    return {};
}

