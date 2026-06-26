import QtQuick
import QtQuick.Window
import QtQuick.Controls
import QtQuick.Layouts

Window {
    id: root
    title: "Folio"
    visible: true

    height: 750
    width: 750

    RowLayout {
        anchors.fill: parent

        Sidebar {}

        TransactionTable {}
    }
}
