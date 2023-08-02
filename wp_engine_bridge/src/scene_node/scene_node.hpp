#ifndef __SCENE_NODE_HPP
#define __SCENE_NODE_HPP

#include <QtQuick/QSGGeometryNode>

#include "scene_geometry.hpp"
#include "scene_material.hpp"

class SceneNode : public QSGGeometryNode {
private:
  SceneMaterial *__material;
  SceneGeometry *__geometry;

public:
  SceneNode();
  ~SceneNode();

  void updateState(const QRectF &rect, const QColor colors[3]);
};

#endif // __SCENE_NODE_HPP
