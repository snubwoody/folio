import QtQuick 6.10
import QtQuick.Controls 6.10
import QtQuick.Layouts 1.3
import App 1.0
import "."

TabButton {
    id: control
    property ButtonGroup group: null
    ButtonGroup.group: group
    checked: selected
    leftPadding: 12
    rightPadding: 12
    topPadding: 4
    bottomPadding: 4
    contentItem: Text {
        text: control.text
        color: control.checked ? Colors.white : Colors.textBody
    }
    background: Rectangle {
        color: control.checked ? Colors.surfacePrimary : control.hovered ? Colors.neutral50 : "transparent"
        radius: 4
    }
}
