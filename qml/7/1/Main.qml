import QtQuick.Layouts
import QtQuick.Controls
import QtQuick.Window
import Felgo


Window { 
    id: root
    width: 320
    height: 640
    visible: true

    ColumnLayout {
        anchors.centerIn: parent
        width: 250
        TextField {
            id: usernameField
            placeholderText: "Username"
            font.pixelSize: 16
        }
        TextField {
            id: passwordField
            placeholderText: "Password"
            font.pixelSize: 16
            echoMode: TextInput.Password
        }
        RowLayout {
            Button { text: "Log In" }
            Button { 
                anchors.right: parent.right
                leftPadding: 30
                rightPadding: 30
                text: "Clear"
                background: Rectangle {
                    color: root.color
                    anchors.fill: parent
                }
                onClicked: {
                    usernameField.text = ""
                    passwordField.text = ""
                }
            }
        }
    }
}