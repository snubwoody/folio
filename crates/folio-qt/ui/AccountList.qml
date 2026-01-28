import QtQuick 6.10
import QtQuick.Controls 6.10
import QtQuick.Layouts 1.3
import QtQuick.Controls.FluentWinUI3
import App 1.0

ListView {
    Layout.fillWidth: true
    Layout.preferredHeight: 200
    // anchors.fill: parent
    orientation: ListView.Horizontal
    spacing: 12
    model: AccountListModel {
        id: accountListModel
        Component.onCompleted: {
            load_accounts();
        }
    }

    delegate: Pane {
        padding: 12
        background: Rectangle {
            color: "white"
            radius: 8
            border.width: 1
            border.color: "grey"
        }
        Column {
            spacing: 8
            Text {
                text: name
            }
            Text {
                text: balance
            }
        }
    }
}
