import QtQuick 6.10
import QtQuick.Controls 6.10
import QtQuick.Layouts 1.3
import QtQuick.Controls.FluentWinUI3
import Qt.labs.qmlmodels
import App 1.0

ColumnLayout {
    RowLayout {
        Text {
            text: "Accounts"
        }
        Item {
            Layout.fillWidth: true
        }
        Button {
            text: "New"
            onClicked: {
                appState.add_account();
                console.log("Clicked!");
            }
        }
    }

    AccountList {}
}
