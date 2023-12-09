import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Window
import Felgo


Window {
    id: root
    width: 320
    height: 640
    visible: false

    SwipeView {
        id: swipe_view

        currentIndex: 1
        anchors.fill: parent

        ColoredPage { id: redPage; pageColor: "red" }
        ColoredPage { id: yellowPage; pageColor: "yellow" }
        ColoredPage { id: greenPage; pageColor: "green" }
    }

    Rectangle {
        color: "gray"

        width: 60
        height: 20
        radius: 5

        anchors.bottom: parent.bottom
        anchors.bottomMargin: 10
        anchors.horizontalCenter: parent.horizontalCenter

        PageIndicator {
            id: page_indicator
            count: swipe_view.count
            currentIndex: swipe_view.currentIndex
            anchors.centerIn: parent

            delegate: Rectangle {
                height: 10
                width: 10
                radius: 5

                color: "white"

                opacity: index === swipe_view.currentIndex ? 1.0 : 0.5
            }

        }
    }
}
