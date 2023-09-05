#ifndef __SCENE_GEOMETRY_HPP
#define __SCENE_GEOMETRY_HPP

#include <QtQuick/QSGGeometry>

/**
 * @brief Struct representing a 3D point with texture coordinates.
 *
 * This structure has attributes to represent the position in 3D space (x, y, z)
 * as well as texture coordinates (u, v).
 */
struct Point3DWithTexCoord {
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
} __attribute__((packed));

/**
 * @brief Provides geometry attributes for a 3D point with texture coordinates.
 *
 * @return A reference to QSGGeometry::AttributeSet for the point with texture
 * coordinates.
 */
inline QSGGeometry::AttributeSet &attr_point3d_with_tex_coord() {
  static QSGGeometry::Attribute attributes[] = {
      QSGGeometry::Attribute::create(0, 3, GL_FLOAT, true), // a_Position
      QSGGeometry::Attribute::create(1, 2, GL_FLOAT, false) // a_TexCoord
  };

  static QSGGeometry::AttributeSet attr_set = {2, sizeof(Point3DWithTexCoord),
                                               attributes};

  return attr_set;
}

/**
 * @brief A custom geometry class derived from QSGGeometry for scenes.
 *
 * It is used to describe a geometry that is used within the scene.
 */
class SceneGeometry : public QSGGeometry {
public:
  explicit SceneGeometry();
  void update(const QRectF &rect);
};

// ********************************
// * SceneGeometry implementation *
// ********************************
SceneGeometry::SceneGeometry()
    : QSGGeometry(attr_point3d_with_tex_coord(), 6) {}

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
