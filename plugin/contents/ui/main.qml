import QtQuick 2.1
import QtQuick.Layouts 1.1
import QtQuick.Window 2.15

import org.kde.plasma.core 2.0 as PlasmaCore

import "WallpaperEngineKDE" 1.0

Rectangle {
    visible: true

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
            renderer.timer_inc(0.016);
        }
    }

}
