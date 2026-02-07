import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Basic
import ".."

// TODO: change to image for better quality
Button {
    id: control
    leftPadding: 12
    rightPadding: 12
    bottomPadding: 8
    topPadding: 8
    
    // property int size: 20
    // property color color: Colors.textBody

    TextLabel {
        text: text
    }

    background: Rectangle {
        color: control.pressed ? Colors.surfacePrimaryActive : control.hovered ? 
        Colors.surfacePrimaryHover : Colors.surfacePrimary
        radius: 12

        Behavior on color {
            ColorAnimation {
                duration: 150
            }
        }
    }
}
