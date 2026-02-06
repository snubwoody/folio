import QtQuick 6.10
import QtQuick.Layouts 1.3
import QtQuick.Controls 6.10
import App 1.0
import Qt.labs.qmlmodels 1.0
import "."
import "components"

TableView {
    id: table
    Layout.fillWidth: true
    Layout.fillHeight: true
    clip: true
    editTriggers: TableView.SingleTapped
    selectionMode: TableView.SingleSelection
    model: transactionModel

    delegate: delegateChooser

    DelegateChooser {
        id: delegateChooser
        role: "column"

        DelegateChoice {
            column: 3
            delegate: Rectangle {
                color: "white"
                border.color: table.selectionModel && table.selectionModel.isSelected(model.index) ? Colors.borderFocus : Colors.borderNeutral
                border.width: 1
                implicitWidth: TableView.view.width / 4
                implicitHeight: 50

                TextLabel {
                    text: display
                }

                TableView.editDelegate: TextField {
                    id: amountTextField
                    anchors.fill: parent
                    text: display
                    TableView.onCommit: {
                        console.log("hi");
                        transactionModel.edit_amount(id,amountTextField.text)
                    }
                }
            }
        }

        // Default has to come last
        DelegateChoice {
            delegate: Rectangle {
                color: "white"
                border.color: table.selectionModel && table.selectionModel.isSelected(model.index) ? Colors.borderFocus : Colors.borderNeutral
                // border.width: 1
                implicitWidth: TableView.view.width / 4
                implicitHeight: 50

                TextLabel {
                    text: display
                }
            }
        }
    }
}
