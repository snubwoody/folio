import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls

// TODO: add focus border color for button
ColumnLayout {
    Layout.alignment: Qt.AlignTop
    Layout.fillHeight: true
    Layout.fillWidth: true
    Layout.maximumWidth: 250

    Text {
        text: qsTr("Spending")
    }
    Text {
        text: qsTr("Analytics")
    }
    RowLayout {
        Layout.fillWidth: true

        Text {
            text: qsTr("Accounts")
        }
        Item {
            Layout.fillWidth: true
        }
        Text {
            text: "K0.00"
        }
    }
    ListView {
        id: listView

        Layout.fillWidth: true
        implicitHeight: 200
        model: accountModel

        delegate: RowLayout {
            width: listView.width

            Text {
                text: display
            }
            Item {
                Layout.fillWidth: true
            }
            Text {
                text: "K0.00"
            }
        }
    }
    Button {
        id: control

        Layout.fillWidth: true
        text: qsTr("Add account")

        background: Rectangle {
            color: "#FFFFFF"
            radius: 2
        }
        contentItem: Text {
            color: "#000000"
            horizontalAlignment: Text.AlignHCenter
            text: control.text
        }

        onClicked: popup.open()
    }


    Popup {
        id: popup

        closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutsideParent
        focus: true
        implicitHeight: 200
        modal: true
        implicitWidth: parent.width
        x: control.x
        y: control.y + control.height
        dim: false

        ColumnLayout {
            anchors.fill: parent
            spacing: 8

            TextField {
                id: accountName
                placeholderText: qsTr("Account name")
            }

            TextField {
                id: startingBalance
                placeholderText: qsTr("Starting balance")
            }
            Button {

                width: parent.width
                text: qsTr("Confirm")

                background: Rectangle {
                    color: "#FFFFFF"
                    radius: 2
                }
                contentItem: Text {
                    color: "#000000"
                    horizontalAlignment: Text.AlignHCenter
                    text: control.text
                }

                onClicked: {
                    accountModel.addAccount(accountName.text,parseFloat(startingBalance.text) || 0)

                    accountName.clear()
                    startingBalance.clear()

                    popup.close()
                }

            }
        }
    }
}
