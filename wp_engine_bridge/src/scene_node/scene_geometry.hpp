#ifndef __SCENE_GEOMETRY_HPP
#define __SCENE_GEOMETRY_HPP

#include <QtQuick/QSGGeometry>

__attribute__((packed)) struct Point3DWithTexCoord {
  float x;
  float y;
  float z;

  float u;
  float v;

  void set(float x, float y, float z, float u, float v) {
    this->x = x;
    this->y = y;
    this->z = z;

    this->u = u;
    this->v = v;
  }
};

inline QSGGeometry::AttributeSet &AttrColoredPoint3DWithTexCoord() {
  static QSGGeometry::Attribute attributes[] = {
      QSGGeometry::Attribute::create(0, 3, GL_FLOAT, true), // a_Position
      QSGGeometry::Attribute::create(1, 2, GL_FLOAT, false) // a_TexCoord
  };

  static QSGGeometry::AttributeSet attrSet = {2, sizeof(Point3DWithTexCoord),
                                              attributes};

  return attrSet;
}

class SceneGeometry : public QSGGeometry {
public:
  SceneGeometry();

  void update(const QRectF &rect);
};

SceneGeometry::SceneGeometry()
    : QSGGeometry(AttrColoredPoint3DWithTexCoord(), 6) {}

void SceneGeometry::update(const QRectF &rect) {
  this->setDrawingMode(GL_TRIANGLES);

  auto *vertices = (Point3DWithTexCoord *)this->vertexData();

  vertices[0].set(rect.right(), rect.top(), 0.0, 1.0, 0.0);
  vertices[1].set(rect.right(), rect.bottom(), 0.0, 1.0, 1.0);
  vertices[2].set(rect.left(), rect.top(), 0.0, 0.0, 0.0);

  vertices[3].set(rect.left(), rect.bottom(), 0.0, 0.0, 1.0);
  vertices[4].set(rect.left(), rect.top(), 0.0, 0.0, 0.0);
  vertices[5].set(rect.right(), rect.bottom(), 0.0, 1.0, 1.0);
}
#endif // __SCENE_GEOMETRY_HPP