import QtQuick 6.10
import QtQuick.Controls 6.10
import QtQuick.Layouts 1.3
import Qt.labs.qmlmodels
import App 1.0
import "."

TableView {
    Layout.fillWidth: true
    Layout.fillHeight: true
    columnSpacing: 1
    rowSpacing: 1
    clip: true

    model: appState.transactions

    delegate: Rectangle {
        implicitWidth: TableView.view.width / 4
        implicitHeight: 50
        border.width: 1
        border.color: Colors.borderNeutral

        Text {
            text: display
        }
    }
}
