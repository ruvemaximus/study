import QtQuick.Controls
import QtQuick.Window
import QtQuick.Layouts
import Felgo

Button { 
    property int value: 0
    text: value.toString()
    onClicked: passwordField.text += value.toString()
}