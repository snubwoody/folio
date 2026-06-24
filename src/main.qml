import QtQuick 2.15
import QtQuick.Window 2.2
import QtQuick.Controls 2.15
import Qt.labs.qmlmodels 1.0

Window {
    id: root
    visible: true
    width: 640
    height: 480
    title: qsTr("Folio")

    TableView {
        anchors.fill: parent
        columnSpacing: 1
        rowSpacing: 1
        clip: true

        model: transactionTableModel

        delegate: Rectangle {
            implicitWidth: 100
            implicitHeight: 50
            Text {
                text: display
            }
        }
    }
}
