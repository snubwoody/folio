import QtQuick 6.10
import QtQuick.Controls 6.10
import QtQuick.Layouts 1.3
import QtQuick.Controls.FluentWinUI3
import Qt.labs.qmlmodels
import App 1.0

TableView {
    Layout.fillWidth: true
    Layout.fillHeight: true
    columnSpacing: 1
    rowSpacing: 1
    clip: true

    model: TableModel {
        TableModelColumn {
            display: "Category"
        }
        TableModelColumn {
            display: "Account"
        }
        TableModelColumn {
            display: "Date"
        }
        TableModelColumn {
            display: "Amount"
        }

        rows: [
            {
                "Category": "Finances",
                "Account": "RBC Savings",
                "Date": "2025-12-12",
                "Amount": "40.00"
            },
            {
                "Category": "Finances",
                "Account": "RBC Savings",
                "Date": "2025-12-12",
                "Amount": "40.00"
            },
            {
                "Category": "Finances",
                "Account": "RBC Savings",
                "Date": "2025-12-12",
                "Amount": "40.00"
            },
        ]
    }

    delegate: Rectangle {
        implicitWidth: 100
        implicitHeight: 50
        border.width: 1

        Text {
            text: display
            anchors.centerIn: parent
        }
    }
}
