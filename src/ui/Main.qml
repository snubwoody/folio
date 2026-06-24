import QtQuick
import QtQuick.Window
import QtQuick.Controls
import QtQuick.Layouts

Window {
    id: root
    title: "Folio"
    visible: true

    height: 750
    width: 750

    RowLayout {
        anchors.fill: parent

        Sidebar {}

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true

            HorizontalHeaderView {
                id: horizontalHeader

                anchors.left: parent.left
                anchors.right: parent.right
                anchors.top: parent.top
                clip: true
                syncView: tableView
            }

            TableView {
                id: tableView
                anchors.top: horizontalHeader.bottom
                anchors.bottom: parent.bottom
                anchors.left: parent.left
                anchors.right: parent.right
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
