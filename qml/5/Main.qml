import QtQuick 2.11
import QtQuick.Controls 2.4
import QtQuick.Layouts 1.11
import QtQuick.Window

Window {
    visible: true
    color: "#ccc"

    Item {
        anchors.fill: parent
        height: 640
        width: 400
        id: root
        states: [
            State {
                name: "main"
                PropertyChanges { target: header; text: "Header" }
                PropertyChanges { target: content; text: "Some content" }
                PropertyChanges { target: backBtn; x: 0 - root.width / 10 }
            },
            State {
                name: "1"
                PropertyChanges { target: header; text: "Header 1" }
                PropertyChanges { target: content; text: "Item 1 content" }
                PropertyChanges { target: navbarBtns.itemAt(0); opacity: 1 }
            },
            State {
                name: "2"
                PropertyChanges { target: header; text: "Header 2" }
                PropertyChanges { target: content; text: "Item 2 content" }
                PropertyChanges { target: navbarBtns.itemAt(1); opacity: 1 }
            },
            State {
                name: "3"
                PropertyChanges { target: header; text: "Header 3" }
                PropertyChanges { target: content; text: "Item 3 content" }
                PropertyChanges { target: navbarBtns.itemAt(2); opacity: 1 }
            }
        ]
        state: "main"

        transitions: [
            Transition {
                PropertyAnimation { properties: "opacity" }
            },
            Transition {
                from: "main"; to: "*"
                PropertyAnimation { properties: "x" }
            },
            Transition {
                from: "*"; to: "main"
                PropertyAnimation { properties: "x" }
            }
        ]

        ColumnLayout {
            spacing: 0
            anchors.fill:parent
            Rectangle {
                width: root.width
                height: 60
                color: "#ccc"

                Text {
                    id: header
                    text:"Header"
                    anchors.centerIn: parent
                }

                Rectangle {
                    width: parent.width / 10
                    height: parent.height
                    x: 0
                    id: backBtn
                    color: "blue"
                    
                    Text { text: "Back"; anchors.centerIn: parent }

                    MouseArea {
                        anchors.fill: parent
                        onClicked: {
                            root.state = "main"
                        }
                    }
                }
            }
            Rectangle {
                width: root.width
                Layout.fillHeight: true

                Rectangle {
                    width: parent.width - 15
                    height: parent.height - 15
                    anchors.centerIn: parent
                    border {
                        width:1; color:"#ccc"
                    }
                    Text {
                        id: content
                        text:"Content"
                        anchors.centerIn: parent
                    }
                }
            }
            Rectangle {
                id: navbar
                height:50
                color:"#eee"
                width: root.width

                RowLayout {
                    anchors.fill:parent
                    spacing:2
                    Repeater {
                        id: navbarBtns
                        model:3

                        Rectangle {
                            width: root.width/3-2
                            height: parent.height
                            color: "#ccc"
                            opacity: 0.5

                            Text {
                                anchors.centerIn: parent
                                text: index + 1
                            }

                            MouseArea {
                                anchors.fill: parent
                                onClicked: {
                                    if (index === 0) {
                                        root.state = "1"
                                    } else if (index === 1) {
                                        root.state = "2"
                                    } else if (index === 2) {
                                        root.state = "3"
                                    } else {
                                        root.state = "main"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}