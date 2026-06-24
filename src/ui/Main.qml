import QtQuick
import QtQuick.Window
import QtQuick.Controls
import QtQuick.Layouts

Window {
    id: root

    height: 480
    title: "Folio"
    visible: true
    width: 640

    RowLayout {
        anchors.fill: parent
        Column {
            id: sidebar
            Layout.preferredWidth: 250
            Layout.fillHeight: true

            Text {
                text: "Spending"
            }
            Text {
                text: "Analytics"
            }
        }

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true

            HorizontalHeaderView {
                id: horizontalHeader

                anchors.left: tableView.left
                anchors.right: parent.right
                anchors.top: parent.top
                clip: true
                syncView: tableView
            }

            TableView {
                id: tableView
                anchors.bottom: parent.bottom
                anchors.left: horizontalHeader.left
                anchors.right: parent.right
                anchors.top: horizontalHeader.bottom
                clip: true
                model: transactionTableModel

                delegate: Rectangle {
                    color: palette.base
                    implicitHeight: 50
                    implicitWidth: 100

                    Text {
                        text: display
                    }
                }
            }
        }
    }
}
