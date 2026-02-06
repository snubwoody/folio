from PySide6.QtCore import Qt, QAbstractListModel, QByteArray, Slot
from dataclasses import  dataclass

@dataclass
class Transaction:
    id: str = ""

class TransactionTableModel(QAbstractListModel):
    ID_ROLE: int = Qt.ItemDataRole.UserRole + 1
    def __init__(self,parent = None):
        super().__init__(parent)
        self.transactions: list[Transaction] = []

    def rowCount(self, parent = None) :
        return len(self.transactions)
    
    def roleNames(self):
        return {
            self.ID_ROLE: QByteArray(b"id"),
        }

    @Slot()
    def load_transactions(self):
        self.beginResetModel()
        self.endResetModel()
