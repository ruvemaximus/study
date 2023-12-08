import QtQuick 2.7
import QtQuick.Window 2.7
import QtQuick.Controls 2.5
import QtQuick.Layouts 1.2
import Felgo 3.0

RoundButton {
    property alias content: btn.text

    id: btn 
    radius: 4
}
