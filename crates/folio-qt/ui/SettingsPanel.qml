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
        anchors.fill: parent
        ColumnLayout {
            // TODO: add max width
            Layout.fillHeight: true
            ButtonGroup {
                id: tabs
            }
            SettingsTabButton {
                id: generalBtn
                Layout.fillWidth: true
                checked: true
                text: "General"
                group: tabs
            }
            SettingsTabButton {
                id: categoriesBtn
                Layout.fillWidth: true
                checked: true
                text: "Categories"
                group: tabs
            }
            SettingsTabButton {
                id: streamsBtn
                Layout.fillWidth: true
                checked: true
                text: "Income streams"
                group: tabs
            }
        }

        StackLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            currentIndex: {
                if (categoriesBtn.checked)
                    return 1;
                if (streamsBtn.checked)
                    return 2;
                return 0;
            }
            GeneralSettingsPanel {}
            CategoriesSettingsPanel {}
            StreamsSettingsPanel {}
        }
    }
}
