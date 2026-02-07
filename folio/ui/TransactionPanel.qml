import QtQuick 6.10
import QtQuick.Controls 6.10
import QtQuick.Layouts 1.3
import App 1.0
import "components"

ColumnLayout {
    Layout.fillWidth: true
    Layout.fillHeight: true
    RowLayout {
        TextLabel {
            text: qsTr("Transactions")
        }
        Item {
            Layout.fillWidth: true
        }
        
        AppButton{
            text: qsTr("New")
        }

        Button {
            text: qsTr("New")
            onClicked: {
                transactionModel.add_expense();
            }
        }
    }

    TransactionTable {
        id: tableView
    }
}
