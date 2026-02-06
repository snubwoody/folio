import sys
from PySide6.QtGui import QGuiApplication
from PySide6.QtQml import QQmlApplicationEngine,qmlRegisterType
from models.account_list import AccountListModel
from models.transaction_table import TransactionTableModel
from models.category_list import CategoryListModel

def main():
    # TODO: add fonts
    print("Running folio")
    app = QGuiApplication(sys.argv)
    qmlRegisterType(AccountListModel,"App",1,0,"AccountListModel")
    qmlRegisterType(CategoryListModel,"App",1,0,"CategoryListModel")
    qmlRegisterType(TransactionTableModel,"App",1,0,"TransactionTableModel")
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