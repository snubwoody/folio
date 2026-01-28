import QtQuick 6.10
import QtQuick.Controls 6.10
import QtQuick.Layouts 1.3
import QtQuick.Controls.FluentWinUI3

ApplicationWindow {
    visible: true
    width: 800
    height: 600
    title: "Folio"
    color: "white"

    Pane {
        padding: 16
        background: Rectangle {
            width: 200
            height: 100
            color: "white"
            radius: 8
            border.width: 1
            border.color: "black"
        }
        Column {
            spacing: 8
            Text {
                text: "RBC Chequing"
            }
            Text {
                text: "CA $0.00"
            }
        }
    }
}
