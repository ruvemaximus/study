import QtQuick 2.7
import QtQuick.Window 2.7
import QtQuick.Controls 2.5
import QtQuick.Layouts 1.2
import Felgo 3.0

Page {
    id: root

    property alias backgroundColor: bg.color

    Button {
      visible: stack_view.depth > 1
      text: "Back"
      onClicked: {
        stack_view.pop()
      }
    }

    background: Rectangle {
        id: bg
    }
}