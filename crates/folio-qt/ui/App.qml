import QtQuick 6.10
import QtQuick.Controls 6.10
import QtQuick.Layouts 1.3
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

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 32
        AccountPanel {}
        TransactionPanel {}
    }
}
