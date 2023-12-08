import QtQuick
import Felgo


Rectangle {
    id: pixel

    property alias pixelColor: pixel.color

    border.width: 1
    border.color: "black"

    height: 50 
    width: 50

    anchors.left: parent.right
}
