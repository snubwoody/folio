import QtQuick 6.10
import QtQuick.Controls 6.10
import QtQuick.Layouts 1.3
import QtQuick.Controls.Basic
import App 1.0
import "."

ColumnLayout {
    Layout.fillWidth: true
    Layout.fillHeight: true

    RowLayout {
        ColumnLayout {
            spacing: 4
            Text {
                text: "Categories"
            }
            Text {
                text: "Categories are used for organising expenses"
            }
        }
        Item {
            Layout.fillWidth: true
        }
        IconButton {
            source: "qrc:/icons/plus.svg"
            onClick: categoryModel.add_category()
        }
    }


    ListView {
        Layout.fillWidth: true
        Layout.fillHeight: true
        clip: true
        boundsBehavior: Flickable.StopAtBounds
        spacing: 12

        model: categoryModel

        delegate: RowLayout {
            // FIXME: warning: TypeError: Cannot read property 'width' of null
            width: parent.width
            EditableText{
                id: textField
                text: title
                onEdited: categoryModel.edit_category(id, textField.text)
            }
            Item {
                Layout.fillWidth: true
            }
            IconButton {
                source: "qrc:/icons/trash-2.svg"
                onClick: categoryModel.delete_category(id)
            }
        }

    }
}



