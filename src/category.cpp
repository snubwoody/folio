#include "category.h"
#include <span>

namespace folio{
    QVariant CategoryModel::data(const QModelIndex &index, int role) const {
        if (role != Qt::DisplayRole){
            return QVariant();
        }

        const auto category = categories[index.row()];

        return QString::fromStdString(category.title);
    }

    void CategoryModel::loadCategories(std::span<Category> categories){
        for (const auto &category : categories) {
            this->categories.push_back(category);
        }
    }

    int CategoryModel::rowCount(const QModelIndex &index ) const {
        return categories.size();
    }

    std::optional<Category> CategoryModel::getCategory(std::string_view id) const{
        for (const auto &category: categories) {
            if (category.id == id){
                return category;
            }
        }

        return {};
    }

}

