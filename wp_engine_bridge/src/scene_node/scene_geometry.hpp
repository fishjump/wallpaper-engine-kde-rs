#ifndef __SCENE_GEOMETRY_HPP
#define __SCENE_GEOMETRY_HPP

#include <QtCore/QVector>
#include <QtGui/QColor>
#include <QtQuick/QSGGeometry>

#include "scene_context.hpp"
#include "texture_spec.hpp"

class SceneGeometry : public QSGGeometry {
public:
  SceneGeometry() : QSGGeometry(AttrColoredPoint3DWithTexCoord(), 0) {}

  void updateState(const QRectF &rect) {
    this->setDrawingMode(GL_TRIANGLES);
    this->allocate(6);

    auto *vertices = (Point3DWithTexCoord *)this->vertexData();

    vertices[0].set(rect.right(), rect.top(), 0.0, 1.0, 0.0);
    vertices[1].set(rect.right(), rect.bottom(), 0.0, 1.0, 1.0);
    vertices[2].set(rect.left(), rect.top(), 0.0, 0.0, 0.0);

    vertices[3].set(rect.left(), rect.bottom(), 0.0, 0.0, 1.0);
    vertices[4].set(rect.left(), rect.top(), 0.0, 0.0, 0.0);
    vertices[5].set(rect.right(), rect.bottom(), 0.0, 1.0, 1.0);
  }
};

#endif // __SCENE_GEOMETRY_HPP
