import sys
from PySide6.QtGui import QGuiApplication
from PySide6.QtQml import QQmlApplicationEngine
from PySide6.QtCore import Qt,QAbstractListModel,QByteArray
from dataclasses import dataclass

@dataclass
class Account:
    id: str
    name: str
    startingBalance: float
    balance: float
    
class AccountListModel(QAbstractListModel):
    ID_ROLE: int = Qt.DisplayRole + 1
    BALANCE_ROLE: int = Qt.DisplayRole + 2
    NAME_ROLE: int = Qt.DisplayRole + 3
    STARTING_BALANCE_ROLE: int = Qt.DisplayRole + 4
    accounts: list[Account]
    def __init__(self,parent = None):
        super().__init__(parent)

    def rowCount(self):
        return len(self.accounts)
    
    def roleNames(self):
        return {
            self.ID_ROLE: QByteArray(b"id"),
            self.NAME_ROLE: QByteArray(b"name"),
            self.BALACNCE_ROLE: QByteArray(b"balance"),
            self.STARTING_BALANCE_ROLE: QByteArray(b"startingBalance"),
        }

def main():
    print("Hello world")
    app = QGuiApplication(sys.argv)
    engine = QQmlApplicationEngine()
    engine.quit.connect(app.quit)
    engine.addImportPath("folio/ui")
    engine.load("folio/ui/App.qml")
    if not engine.rootObjects():
        print("Error: Failed to load application")
        sys.exit(-1)
    sys.exit(app.exec())

if __name__ == "__main__":
    main()