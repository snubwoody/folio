import QtQuick 6.10
import QtQuick.Controls 6.10
import QtQuick.Layouts 1.3
import App 1.0
import "."
import "components"

ApplicationWindow {
    visible: true
    width: 800
    height: 600
    title: "Folio"
    color: "white"

    FontLoader{
        id: satoshiVariable
        source: "qrc:/fonts/Satoshi-Variable.ttf"
    }

    AccountListModel {
        id: accountModel
        Component.onCompleted: {
            load_accounts();
        }
    }

    TransactionTableModel {
        id: transactionModel
        Component.onCompleted: {
            load_expenses();
        }
    }

    CategoryListModel {
        id: categoryModel
        Component.onCompleted: {
            load_categories();
        }
    }

    RowLayout {
        anchors.fill: parent

        Rectangle {
            width: 60
            Layout.fillHeight: true
            color: Colors.neutral25
            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 12
                IconButton {
                    source: "qrc:/icons/home.svg"
                    onClick: {
                        stackLayout.currentIndex = 0;
                    }
                }
                IconButton {
                    source: "qrc:/icons/bar-chart.svg"
                    onClick: stackLayout.currentIndex = 1
                }
                Item {
                    Layout.fillHeight: true
                }
                IconButton {
                    source: "qrc:/icons/settings.svg"
                    onClick: settingsPopup.open()
                }
            }
        }

        SettingsPanel {
            id: settingsPopup
        }

        StackLayout {
            id: stackLayout
            Layout.fillWidth: true
            Layout.fillHeight: true
            HomePage {}
            AnalyticsPage {}
        }
    }
}
