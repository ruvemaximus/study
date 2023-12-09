import QtQuick
import Felgo


Item { 
    property alias pageColor: background.color

    Rectangle {
        id: background
        width: root.width
        height: root.height
    }
}