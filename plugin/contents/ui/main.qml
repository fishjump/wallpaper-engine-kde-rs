import QtQuick 2.1
import QtQuick.Layouts 1.1
import QtQuick.Window 2.15

import org.kde.plasma.core 2.0 as PlasmaCore

import "WallpaperEngineKDE" 1.0

Rectangle {
    visible: true
    width: 1920
    height: 1080

    SceneRenderer {
        id: renderer
        anchors.fill: parent
    }

    Greeter {
        id: greeter;
        // Set a property
        name: wallpaper.configuration.name
    }

    Text {
        anchors.centerIn: parent
        // Call a method
        text: greeter.compute_greetings(wallpaper.configuration.verb)
    }

    Timer {
        id: timer
        interval: 16
        repeat: true
        running: true
        onTriggered: {
            renderer.tick_tock(0.016);
        }
    }
    
    Connections {
        target: Qt.application
        onActiveChanged: {
            if (Qt.application.active) {
                timer.start();
            }
        }
    }

}
