#ifndef __SCENE_GEOMETRY_HPP
#define __SCENE_GEOMETRY_HPP

#include <QtCore/QVector>
#include <QtGui/QColor>
#include <QtQuick/QSGGeometry>

#include <iostream>

class SceneGeometry : public QSGGeometry {

public:
  SceneGeometry()
      : QSGGeometry(QSGGeometry::defaultAttributes_ColoredPoint2D(), 3) {}

  void updateVertexData(const QRectF &rect, const QColor colors[3]) {
    this->setDrawingMode(GL_TRIANGLES);

    auto *vertices = this->vertexDataAsColoredPoint2D();

    // WTF Qt? ColoredPoint2D converts color to float, though I'm setting the
    // "VBO" Qt shouldn't do this implicit conversion. Everyone knows the range
    // is 0 to 1.0
    // Here costs me ~ 1 hour to debug
    vertices[0].set(rect.right() / 2, rect.top(), colors[0].red(),
                    colors[0].green(), colors[0].blue(), colors[0].alpha());
    vertices[1].set(rect.right(), rect.bottom(), colors[1].red(),
                    colors[1].green(), colors[1].blue(), colors[1].alpha());
    vertices[2].set(rect.left(), rect.bottom(), colors[2].red(),
                    colors[2].green(), colors[2].blue(), colors[2].alpha());
  }
};

#endif // __SCENE_GEOMETRY_HPP
