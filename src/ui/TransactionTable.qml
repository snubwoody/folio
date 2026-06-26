import QtQuick
import QtQuick.Window
import QtQuick.Controls
import QtQuick.Layouts

// TODO: make popup appear ontop if it's below the screen
Rectangle {
    Layout.fillHeight: true
    Layout.fillWidth: true

    HorizontalHeaderView {
        id: horizontalHeader

        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: parent.top
        clip: true
        syncView: tableView

        delegate: Rectangle {
            Text {
                color: Colors.textMuted
                font.weight: 600
                text: display
            }
        }
    }
    TableView {
        id: tableView

        anchors.bottom: parent.bottom
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: horizontalHeader.bottom
        clip: true
        model: transactionTableModel

        // delegate: Rectangle {
        //     border.color: Colors.neutral50
        //     border.width: 1
        //
        //     Text {
        //         text: display
        //     }
        // }

        delegate: TableViewDelegate {
            id: tableCell

            background: Item {
                Rectangle {
                    anchors.fill: parent
                    border.color: Colors.neutral50
                    border.width: 1
                    // anchors.margins: 10
                }
                Rectangle {
                    anchors.fill: parent
                    border.color: "darkblue"
                    border.width: tableCell.current ? 2 : 0
                    color: "transparent"
                }
            }

            contentItem: Item {
                RowLayout {
                    anchors.fill: parent

                    CheckBox {
                        id: checkBox

                        // Layout.fillHeight: true
                        checked: false
                        visible: tableCell.column === 0
                    }
                    Text {
                        // Layout.fillHeight: true
                        // Layout.fillWidth: true
                        Layout.leftMargin: 4
                        color: tableCell.selected ? "white" : "black"
                        text: tableCell.model.display
                        verticalAlignment: Text.AlignVCenter
                    }
                }
            }
        }
    }
}
