import QtQuick 2.1
import QtQuick.Layouts 1.1

import org.kde.plasma.core 2.0 as PlasmaCore

import "WallpaperEngineKDE" 1.0

Rectangle {
    visible: true

    PlasmaCore.DataSource {
        id: configData
        engine: "config"
        connectedSources: [PlasmaCore.configurationSource]
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
}
