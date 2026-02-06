import QtQuick 6.10
import QtQuick.Controls 6.10
import QtQuick.Layouts 1.3
import QtQuick.Controls.FluentWinUI3
import App 1.0
import ".."


Text {
    property int size: Style.fontSizeBody
    font.family: satoshiVariable.font.family
    font.pixelSize: size
}