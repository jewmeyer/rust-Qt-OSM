import QtQuick
import QtQuick.Controls
import QtQuick.Window
import QtLocation
import QtPositioning

// This must match the uri and version
// specified in the qml_module in the build.rs script.
import com.kdab.cxx_qt.demo 1.0

ApplicationWindow {
    height: 900
    title: qsTr("Hello World")
    visible: true
    width: 1200
    color: palette.window

    MyObject {
        id: myObject
        number: 1
        string: qsTr("My String with my number: %1").arg(myObject.number)
    }

    Column {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 10

        Label {
            text: qsTr("Number: %1").arg(myObject.number)
            color: palette.text
        }

        Label {
            text: qsTr("String: %1").arg(myObject.string)
            color: palette.text
        }

        Button {
            text: qsTr("Increment Number")

            onClicked: myObject.incrementNumber()
        }

        Button {
            text: qsTr("Say Hi!")

            onClicked: myObject.sayHi(myObject.string, myObject.number)
        }

        Button {
            text: qsTr("Quit")

            onClicked: Qt.quit()
        }

        Map {
            id: map
            width: 1200
            height: 600
            center: QtPositioning.coordinate(49.0, 8.42) // Oslo
            zoomLevel: 13
            property geoCoordinate startCentroid

            plugin: Plugin {
                name: "osm"
                PluginParameter { name: "osm.useragent"; value: "Exploring Rust, CXX-Qt, OSM" }
                // Replace the XXXXXX with a valid API key obtained from thunderforest.com
                PluginParameter {
                    name: "osm.mapping.providersrepository.address";
                    value: "https://tile.thunderforest.com/transport/{z}/{x}/{y}.png?apikey=XXXXXX" 
                }
            }

            PinchHandler {
                id: pinch
                target: null
                onActiveChanged: if (active) {
                    map.startCentroid = map.toCoordinate(pinch.centroid.position, false)
                }
                onScaleChanged: (delta) => {
                    map.zoomLevel += Math.log2(delta)
                    map.alignCoordinateToPoint(map.startCentroid, pinch.centroid.position)
                }
                onRotationChanged: (delta) => {
                    map.bearing -= delta
                    map.alignCoordinateToPoint(map.startCentroid, pinch.centroid.position)
                }
                grabPermissions: PointerHandler.TakeOverForbidden
            }
            WheelHandler {
                id: wheel
                // workaround for QTBUG-87646 / QTBUG-112394 / QTBUG-112432:
                // Magic Mouse pretends to be a trackpad but doesn't work with PinchHandler
                // and we don't yet distinguish mice and trackpads on Wayland either
                // acceptedDevices: Qt.platform.pluginName === "cocoa" || Qt.platform.pluginName === "wayland"
                //         ? PointerDevice.Mouse | PointerDevice.TouchPad
                //         : PointerDevice.Mouse
                rotationScale: 1/120
                property: "zoomLevel"
            }
            DragHandler {
                id: drag
                target: null
                onTranslationChanged: (delta) => map.pan(-delta.x, -delta.y)
            }
            Shortcut {
                enabled: map.zoomLevel < map.maximumZoomLevel
                sequence: StandardKey.ZoomIn
                onActivated: map.zoomLevel = Math.round(map.zoomLevel + 1)
            }
            Shortcut {
                enabled: map.zoomLevel > map.minimumZoomLevel
                sequence: StandardKey.ZoomOut
                onActivated: map.zoomLevel = Math.round(map.zoomLevel - 1)
            }
        }
    }

}
