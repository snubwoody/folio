#pragma once

#include <QAbstractTableModel>
#include <QQmlEngine>
#include <optional>
#include <span>
#include <string>
#include <vector>

namespace folio {
    struct Category {
        std::string id;
        std::string title;
    };

    class CategoryModel : public QAbstractListModel {
        Q_OBJECT
        QML_ELEMENT

        std::vector<Category> categories;

      public:
        void loadCategories(std::span<Category> categories);

        // Returns the category with the provided id, if it exists.
        std::optional<Category> getCategory(std::string_view id) const;
        int rowCount(const QModelIndex& index = QModelIndex()) const override;

        QVariant data(const QModelIndex& index, int role) const override;

        QHash<int, QByteArray> roleNames() const override {
            return {{Qt::DisplayRole, "display"}};
        }
    };

} // namespace folio
