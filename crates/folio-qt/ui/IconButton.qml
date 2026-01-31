import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Basic

Button {
    id: control

    property string source: ""
    property int size: 20
    property color color: Colors.textBody
    property var onClick: function () {}

    width: size + 8
    height: size + 8

    icon.source: source
    icon.width: size
    icon.height: size
    icon.color: color

    onClicked: onClick

    background: Rectangle {
        color: control. pressed ? Colors.neutral100 : control.hovered ? Colors.neutral50 : Colors.white
        radius: 4

        Behavior on color {
            ColorAnimation { duration: 150 }
        }
    }
}

