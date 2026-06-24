#pragma once

#include <optional>
#include <span>
#include <string>
#include <QQmlEngine>
#include <QAbstractTableModel>
#include <vector>

struct Category {
    std::string id;
    std::string title;
};

class CategoryModel : public QAbstractListModel
{
    Q_OBJECT
    QML_ELEMENT

    std::vector<Category> categories;

public:

    void loadCategories(std::span<Category> categories);

    // Returns the category with the provided id, if it exists.
    std::optional<Category> getCategory(std::string_view id) const;
    int rowCount(const QModelIndex &index = QModelIndex()) const override;

    QVariant data(const QModelIndex &index, int role) const override;

    QHash<int, QByteArray> roleNames() const override {
        return { {Qt::DisplayRole, "display"} };
    }
};
