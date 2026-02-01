import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Basic

// TODO: change to image for better quality
Button {
    id: control

    signal click

    property string source: ""
    property int size: 20
    property color color: Colors.textBody

    width: size + 8
    height: size + 8

    icon.source: source
    icon.width: size
    icon.height: size
    icon.color: color

    // TODO: just use this externally
    onClicked: control.click()

    background: Rectangle {
        color: control.pressed ? Colors.neutral100 : control.hovered ? Colors.neutral50 : Colors.white
        radius: 4

        Behavior on color {
            ColorAnimation {
                duration: 150
            }
        }
    }
}
