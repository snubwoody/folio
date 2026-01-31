import QtQuick 6.10
import QtQuick.Controls 6.10
import QtQuick.Layouts 1.3
// import QtQuick.Controls.FluentWinUI3
import Qt.labs.qmlmodels
import QtQuick.Controls.Basic
import App 1.0
import "."

ApplicationWindow {
    visible: true
    width: 800
    height: 600
    title: "Folio"
    color: "white"

    // Declare global app state
    AppState {
        id: appState
        Component.onCompleted: {
            load_data();
        }
    }

    // Usage:
    IconButton {
        source: "qrc:/icons/plus.svg"
        // iconSize: 20
        // onClicked: deleteItem()
    }

    ColumnLayout {
        anchors.fill: parent
        AccountPanel {}
        TransactionPanel {}
    }
}
