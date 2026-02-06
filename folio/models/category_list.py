from PySide6.QtCore import Qt,QAbstractListModel,QByteArray,Slot
from PySide6.QtQml import  QmlElement
from dataclasses import dataclass

QML_IMPORT_NAME = "BaseModel"
QML_IMPORT_MAJOR_VERSION = 1

@dataclass
class Category:
    id: str = ""
    title: str = ""

@QmlElement
class CategoryListModel(QAbstractListModel):
    ID_ROLE: int = Qt.ItemDataRole.UserRole + 1
    TITLE_ROLE: int = ID_ROLE + 1
    # categories: list[Category] = []

    def __init__(self,parent = None):
        super().__init__(parent)
        self.categories: list[Category] = []

    def rowCount(self, parent = None):
        return len(self.categories)

    def data(self, index, role = Qt.ItemDataRole.DisplayRole):
        if index.row() >= self.rowCount():
            return None
        return ""
    
    def roleNames(self):
        return {
            self.ID_ROLE: QByteArray(b"id"),
            self.TITLE_ROLE: QByteArray(b"title"),
        }

    @Slot()
    def load_categories(self):
        self.beginResetModel()
        self.endResetModel()
