import QtQuick 2.6
import QtQuick.Controls 1.4
import QtQuick.Controls.Styles 1.4
import QtQuick.Layouts 1.11
import QtQuick.Window 2.2
import Rust 1.0

Window {
    visible: true
    width: 480
    height: 480
    title: qsTr("Rust + Qt")

    Calculator {
        id: calc
    }

    ColumnLayout {
        anchors.fill: parent
        Label {
            text: calc.view
            color: "black"
            Layout.fillWidth: true
            Layout.fillHeight: true
            horizontalAlignment: Text.AlignHCenter
            verticalAlignment: Text.AlignVCenter
        }
        GridLayout {
            columns: 3
            rows: 3
            Layout.fillWidth: true
            Layout.fillHeight: true
            layoutDirection: Qt.RightToLeft
            Repeater {
                model: 10
                Button {
                    property int num: (9-index)
                    text: ""+num
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    onClicked: calc.number(num)
                }
            }
            Button {
                text: "="
                Layout.fillWidth: true
                Layout.fillHeight: true
                onClicked: calc.calculate()
            }
            Button {
                text: "CLEAR"
                Layout.fillWidth: true
                Layout.fillHeight: true
                onClicked: calc.clear()
            }
        }
        RowLayout {
            Button {
                text: "+"
                Layout.fillWidth: true
                Layout.fillHeight: true
                onClicked: calc.op(0)
            }
            Button {
                text: "-"
                Layout.fillWidth: true
                Layout.fillHeight: true
                onClicked: calc.op(1)
            }
            Button {
                text: "*"
                Layout.fillWidth: true
                Layout.fillHeight: true
                onClicked: calc.op(2)
            }
            Button {
                text: "/"
                Layout.fillWidth: true
                Layout.fillHeight: true
                onClicked: calc.op(3)
            }
        }
    }
}
