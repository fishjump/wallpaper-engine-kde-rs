/*
* Copyright (C) %{CURRENT_YEAR} by %{AUTHOR} <%{EMAIL}>
*
* This program is free software; you can redistribute it and/or modify
* it under the terms of the GNU Library General Public License as
* published by the Free Software Foundation; either version 2 or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
* GNU General Public License for more details
*
* You should have received a copy of the GNU Library General Public
* License along with this program; if not, write to the
* Free Software Foundation, Inc.,
* 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.
*/

import QtQuick 2.1
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.8

GridLayout {
    id: root
    width: childrenRect.width
    height: childrenRect.height
    columns: 2

    property string cfg_name
    property string cfg_verb

    TextField {
        id: textField
        text: cfg_name

        onTextChanged: {
            cfg_name = text
        }
    }
    Text {
        id: textDisplay
        text: "You typed: " + cfg_name
        horizontalAlignment: Text.AlignHCenter
        verticalAlignment: Text.AlignVCenter
    }

    TextField {
        id: textField2
        text: cfg_verb

        onTextChanged: {
            cfg_verb = text
        }
    }
    Text {
        id: textDisplay2
        text: "You typed: " + cfg_verb
        horizontalAlignment: Text.AlignHCenter
        verticalAlignment: Text.AlignVCenter
    }
}
