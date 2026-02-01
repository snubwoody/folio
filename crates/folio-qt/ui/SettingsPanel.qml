import QtQuick 6.10
import QtQuick.Controls 6.10
import QtQuick.Layouts 1.3
import App 1.0
import "."

Popup {
    modal: true
    parent: Overlay.overlay
    width: parent.width - 192
    height: parent.height - 96
    x: (parent.width - width) / 2
    y: (parent.height - height) / 2
    background: Rectangle {
        color: Colors.white
        radius: 16
    }

    RowLayout {
        ColumnLayout {
            ButtonGroup {
                id: tabs
            }
            SettingsTabButton {
                Layout.fillWidth: true
                id: generalBtn
                checked: true
                text: "General"
                group: tabs
            }
            SettingsTabButton {
                Layout.fillWidth: true
                id: categoriesBtn
                checked: true
                text: "Categories"
                group: tabs
            }
            SettingsTabButton {
                Layout.fillWidth: true
                id: streamsBtn
                checked: true
                text: "Income streams"
                group: tabs
            }
        }

        StackLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            currentIndex: {
                if (categoriesBtn.checked) return 1
                if (streamsBtn.checked) return 2
                return 0
            }
            Text {text:"General"}
            Text {text:"Categories"}
            Text {text:"Income streams"}
        }
    }
}
