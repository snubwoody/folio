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
            model: appState.accounts
            delegate: Column {
                TextField {
                    id: textField
                    text: name
                    color: Colors.textBody
                    onEditingFinished: {
                        console.log("Hi");
                        // TODO: editing twice
                        textField.focus = false;
                    }
                    background: Rectangle {
                        Rectangle {
                            anchors.left: parent.left
                            anchors.right: parent.right
                            anchors.bottom: parent.bottom
                            height: 1
                            color: textField.activeFocus ? Colors.neutral950 : "transparent"
                            opacity: textField.activeFocus ? 1 : 0.0

                            Behavior on height {
                                NumberAnimation {
                                    duration: 120
                                }
                            }
                            Behavior on opacity {
                                NumberAnimation {
                                    duration: 120
                                }
                            }
                        }
                    }
                }
                Row {
                    spacing: 4
                    Text {
                        text: "Starting balance"
                    }
                    Text {
                        text: startingBalance
                    }
                }
            }
        }
    }
}
