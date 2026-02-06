from PySide6.QtCore import Qt,QAbstractListModel,QByteArray,Slot
from dataclasses import dataclass

@dataclass
class Category:
    id: str = ""
    title: str = ""

class CategoryListModel(QAbstractListModel):
    ID_ROLE: int = Qt.ItemDataRole.UserRole + 1
    TITLE_ROLE: int = ID_ROLE + 1

    def __init__(self,parent = None):
        super().__init__(parent)
        self.categories: list[Category] = []

    def rowCount(self, parent = None):
        return len(self.categories)

    def data(self, index, role):
        if index.row() >= self.rowCount():
            return None
        category = self.categories[index.row()]
        match role:
            case self.ID_ROLE:
                return category.id
            case self.TITLE_ROLE:
                return category.title
        return None
    
    def roleNames(self):
        return {
            self.ID_ROLE: QByteArray(b"id"),
            self.TITLE_ROLE: QByteArray(b"title"),
        }

    @Slot()
    def load_categories(self):
        self.beginResetModel()
        self.categories: list[Category] = [
            Category(title="Dining out"),
            Category(title="Entertainment"),
            Category(title="School"),
            Category(title="Transport"),
            Category(title="Transit"),
            Category(title="Groceries"),
            ]
        self.endResetModel()
