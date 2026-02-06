from PySide6.QtCore import Qt, QAbstractListModel, QAbstractTableModel, QByteArray, Slot
from dataclasses import  dataclass
from folio.models.category_list import Category
from folio.models.account_list import Account

class Expense:
    def __init__(
        self,
        id: str = "",
        account: Account = Account(),
        category: Category = Category(),
        date: str = "",
        amount: float = 0
    ):
        self.id = id
        self.account: Account = account
        self.category: Category = category
        self.date: str = date
        self.amount: float = amount

class TransactionTableModel(QAbstractTableModel):
    COLUMN_COUNT: int = 4

    def __init__(self,parent = None):
        super().__init__(parent)
        self.expenses: list[Expense] = []

    def rowCount(self, parent = None):
        return len(self.expenses)
    
    def columnCount(self, parent = None):
        return self.COLUMN_COUNT
    
    def data(self, index,role=Qt.ItemDataRole.DisplayRole):
        if index.row() >= len(self.expenses) or index.column() >= self.columnCount():
            return None
        
        expense = self.expenses[index.row()]
        match index.column():
            case 0:
                return expense.category.title
            case 1:
                return expense.account.name
            case 2:
                return expense.date
            case 3:
                return expense.amount
        return None
    
    @Slot()
    def load_expenses(self):
        self.beginResetModel()
        self.expenses = [
            Expense(amount=0,date="2024-12-12",account=Account(name="RBC"),category=Category(title="Name")),
            Expense(amount=550,date="2024-12-12",account=Account(name="Absa"),category=Category(title="Name")),
            Expense(amount=330,date="2024-12-12",account=Account(name="FNB"),category=Category(title="Name")),
            Expense(amount=240,date="2024-12-12",account=Account(name="Zanax"),category=Category(title="Name")),
            Expense(amount=240,date="2024-12-12",account=Account(name="RBC"),category=Category(title="Name")),
            Expense(amount=120,date="2024-12-12",account=Account(name="RBC"),category=Category(title="Name")),
        ]
        self.endResetModel()
