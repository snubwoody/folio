// IconButton.qml
import QtQuick 2.15
import QtQuick.Controls 2.15

Button {
    id: control

    property string source: ""
    property int size: 16
    property color color: Colors.textBody

    width: size + 8
    height: size + 8

    icon.source: source
    icon.width: size
    icon.height: size
    icon.color: color

    // background: Rectangle {
    //     color: control.pressed ? "#d0d0d0" :
    //         control.hovered ? "#f0f0f0" : "transparent"
    //     radius: 4
    // }
}

