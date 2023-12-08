import QtQuick 2.7
import QtQuick.Window 2.7
import QtQuick.Controls 2.5
import QtQuick.Layouts 1.2
import Felgo 3.0


Window {
  width: 360
  height: 640
  visible: true

  function goToPage(page) {
    if (stack_view.pop(page) === null) {
      stack_view.push(page)
    }
  }

  StackView {
    id: stack_view
    anchors.fill: parent
    initialItem: green_page
  }

  TrafficLightPage {
    id: green_page
    backgroundColor: "green"

    NavBar {
      NavButton {
        content: "Yellow"
        onClicked: { goToPage(yellow_page) }
      }
      NavButton {
        content: "Red"
        onClicked: { goToPage(red_page) }
      }
    }
  }

  TrafficLightPage {
    id: yellow_page
    visible: false
    backgroundColor: "yellow"

    NavBar {
      NavButton {
        content: "Green"
        onClicked: { goToPage(green_page) }
      }
      NavButton {
        content: "Red"
        onClicked: { goToPage(red_page) }
      }
    }
  }

  TrafficLightPage {
    id: red_page
    visible: false
    backgroundColor: "red"

    NavBar {
      NavButton {
        content: "Yellow"
        onClicked: { goToPage(yellow_page) }
      }
      NavButton {
        content: "Green"
        onClicked: { goToPage(green_page) }
      }
    }
  }
}