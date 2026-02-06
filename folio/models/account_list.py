from PySide6.QtCore import Qt, QAbstractListModel, QByteArray, Slot, QModelIndex
from dataclasses import dataclass

@dataclass
class Account:
    id: str = ""
    name: str = ""
    startingBalance: float = 0
    balance: float = 0
    
class AccountListModel(QAbstractListModel):
    ID_ROLE: int = Qt.ItemDataRole.UserRole + 1
    BALANCE_ROLE: int = ID_ROLE + 1
    NAME_ROLE: int = BALANCE_ROLE + 3
    STARTING_BALANCE_ROLE: int = NAME_ROLE + 4

    def __init__(self,parent = None):
        super().__init__(parent)
        self.accounts: list[Account] = []

    def rowCount(self, parent = None):
        return len(self.accounts)

    def data(self, index: QModelIndex, role = ID_ROLE):
        if index.row() >= len(self.accounts):
            return None

        account = self.accounts[index.row()]

        return None

    def roleNames(self):
        return {
            self.ID_ROLE: QByteArray(b"id"),
            self.NAME_ROLE: QByteArray(b"name"),
            self.BALANCE_ROLE: QByteArray(b"balance"),
            self.STARTING_BALANCE_ROLE: QByteArray(b"startingBalance"),
        }

    @Slot()
    def load_accounts(self):
        self.beginResetModel()
        self.accounts = [
            Account(name="FNB Savings",balance=500),
            Account(name="Absa Chequing",balance=3500),
        ]
        self.endResetModel()
