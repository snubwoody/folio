import QtQuick 2.15
import QtQuick.Window 2.2
import QtQuick.Controls 2.15
import Qt.labs.qmlmodels 1.0

Window {
    id: root
    height: 480
    title: qsTr("Folio")
    visible: true
    width: 640

    Rectangle {
        anchors.fill: parent

        HorizontalHeaderView{
            id: horizontalHeader
            syncView: tableView
            anchors.left: tableView.left
            anchors.top: parent.top
            anchors.right: parent.right
            clip: true
        }

        TableView {
            id: tableView
            anchors.top: horizontalHeader.bottom
            anchors.left: horizontalHeader.left
            anchors.bottom: parent.bottom
            anchors.right: parent.right

            clip: true
            columnSpacing: 1
            model: transactionTableModel
            rowSpacing: 1

            delegate: Rectangle {
                implicitHeight: 50
                implicitWidth: 100
                color: palette.base

                Text {
                    text: display
                }
            }
        }
    }
}
