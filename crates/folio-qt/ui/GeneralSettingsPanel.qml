import QtQuick 6.10
import QtQuick.Controls 6.10
import QtQuick.Layouts 1.3
import QtQuick.Controls.Basic
import App 1.0
import "."

ColumnLayout {
    Layout.fillWidth: true
    Layout.fillHeight: true
    RowLayout {
        Layout.fillWidth: true
        ColumnLayout {
            spacing: 8
            Text {
                text: "Currency code"
            }
            Text {
                text: "The ISO currency code"
            }
        }
        Item {
            Layout.fillWidth: true
        }
        Text {
            text: "CAD"
        }
    }
    ColumnLayout {
        spacing: 6
        Layout.fillHeight: true
        Text {
            text: "Accounts"
        }
        ListView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.preferredHeight: 500
            spacing: 16
            model: accountModel
            delegate: Column {
                EditableText{
                    id: textField
                    text: name
                    onEdited: accountModel.edit_account(id, textField.text, startingBalance);
                }
                Row {
                    spacing: 4
                    Text {
                        text: "Starting balance"
                    }
                    EditableText{
                        // FIXME: broken
                        id: balanceField
                        text: startingBalance
                        validator: DoubleValidator {
                            bottom: 0.00
                            decimals: 2
                            notation: DoubleValidator.StandardNotation
                        }
                        inputMethodHints: Qt.ImhFormattedNumbersOnly
                        onEdited: accountModel.edit_account(id, textField.text, balanceField.text);
                    }
                }
            }
        }
    }
}
