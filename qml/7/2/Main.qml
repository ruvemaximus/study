import QtQuick.Controls
import QtQuick.Window
import QtQuick.Layouts
import Felgo


Window {
    id: root
    width: 320
    height: 640
    visible: true

    ColumnLayout {
        anchors.centerIn: parent
        spacing: 20
        Text {
            text: "Enter your password:"
            font.pixelSize: 16
            Layout.alignment: Qt.AlignCenter
        }
        Rectangle {
            id: passwordFieldWrapper
            color: "white"
            border.width: 2
            border.color: "black"
            width: parent.width
            height: 50
            Text {
                id: passwordField
                text: passwordField.text
                visible: false
            }
            Layout.alignment: Qt.AlignCenter
            Row {
                spacing: 6
                anchors.centerIn: parent
                Repeater {
                    model:6
                    Label {
                        width: 20
                        height: 20
                        font.pixelSize: 36
                        text: "*"
                        Layout.alignment: Qt.AlignCenter
                        color: index < passwordField.text.length ? "black" : "light gray"
                    }
                }
            }
        }
        GridLayout {
            id: keypad
            rows: 4
            columns: 3
            width: parent.width
            Repeater { model: 3; KeypadBtn { value: index } }
            Repeater { model: 3; KeypadBtn { value: index+4 } }
            Repeater { model: 3; KeypadBtn { value: index+7 } }
            Button { }
            KeypadBtn { value: 0 }
            Button { 
                text: "Clear"
                onClicked: {
                    passwordField.text = ""
                }
            }
        }
        Button {
            text: "Log In"
        }
    }
}