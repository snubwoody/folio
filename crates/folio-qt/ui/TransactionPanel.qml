import QtQuick 6.10
import QtQuick.Controls 6.10
import QtQuick.Layouts 1.3
import QtQuick.Controls.FluentWinUI3
import Qt.labs.qmlmodels
import App 1.0

ColumnLayout {
    Layout.fillWidth: true
    Layout.fillHeight: true
    RowLayout {
        Text {
            text: "Transactions"
        }
        Item {
            Layout.fillWidth: true
        }
        Button {
            id: newAccountButton
            text: "New"
            onClicked: {
                // appState.add_account();
                newAccountPopup.open();
            }
        }

        Popup {
            id: newAccountPopup
            // x: newAccountButton.x + newAccountPopup.width - width
            // y: newAccountButton.y + newAccountButton.height + 12
            // width: 500
            ColumnLayout {
                anchors.fill: parent
                TextField {
                    id: accountName
                    Layout.fillWidth: true
                    placeholderText: "Enter account name"
                }
                TextField {
                    id: accountBalance
                    Layout.fillWidth: true
                    placeholderText: "Enter starting balance"
                    validator: DoubleValidator {
                        bottom: 0.00
                        decimals: 2
                        notation: DoubleValidator.StandardNotation
                    }
                    inputMethodHints: Qt.ImhFormattedNumbersOnly
                }
                Button {
                    text: "Add expense"
                    Layout.fillWidth: true
                    onClicked: {
                        appState.add_account(accountName.text, accountBalance.text);
                        accountName.text = "";
                        accountBalance.text = "";
                        newAccountPopup.close();
                    }
                }
            }
        }
    }

    TransactionTable {}
}
