import QtQuick 6.10
import QtQuick.Controls 6.10
import QtQuick.Layouts 1.3
import QtQuick.Controls.Basic
import App 1.0
import "."

TextField {
    id: control
    color: Colors.textBody
    signal edited
    onEditingFinished: {
        // FIXME: editing twice
        control.edited()
        control.focus = false;
    }
    background: Rectangle {
        Rectangle {
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.bottom: parent.bottom
            height: 1
            color: control.activeFocus ? Colors.neutral950 : "transparent"
            opacity: control.activeFocus ? 1 : 0.0

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
