import QtQuick 6.10
import QtQuick.Layouts 1.3
import App 1.0
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

    delegate: Rectangle {
        color: "white"
        border.color: table.selectionModel && table.selectionModel.isSelected(model.index) ? Colors.borderFocus : Colors.borderNeutral
        border.width: 1
        // padding: 12
        implicitWidth: TableView.view.width / 4
        implicitHeight: 50

        TextLabel {
            text: display
        }

        // TableView.editDelegate: TextField {
        //     anchors.fill: parent
        //     text: display
        //     TableView.onCommit: {
        //         console.log("hi")
        //     }
        // }
    }
}
