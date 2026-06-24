import QtQuick
import QtQuick.Window
import QtQuick.Layouts

ColumnLayout {
    Layout.fillHeight: true
    Layout.fillWidth: true
    Layout.maximumWidth: 250

    Text {
        text: "Spending"
    }
    Text {
        text: "Analytics"
    }

    RowLayout {
        Layout.fillWidth: true

        Text {
            text: "Accounts"
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

        Layout.fillHeight: true
        Layout.fillWidth: true
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
}
