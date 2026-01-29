import QtQuick 6.10
import QtQuick.Controls 6.10
import QtQuick.Layouts 1.3
import QtQuick.Controls.FluentWinUI3
import Qt.labs.qmlmodels
import App 1.0

ApplicationWindow {
    visible: true
    width: 800
    height: 600
    title: "Folio"
    color: "white"

    AppState {
        id: appState
        Component.onCompleted: {
            load_data()
        }
    }

    ColumnLayout {
        anchors.fill: parent
        AccountList {}
        Button {
            text: "New"
            onClicked: {
                console.log("Clicked!")
            }
        }
        TransactionTable {}
    }
}
