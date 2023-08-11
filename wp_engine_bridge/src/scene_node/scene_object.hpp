#ifndef __SCENE_OBJECT_HPP
#define __SCENE_OBJECT_HPP

#include <QtQuick/QSGGeometryNode>

#include "scene_context.hpp"
#include "scene_geometry.hpp"
#include "scene_material.hpp"

class SceneObject : public QSGGeometryNode {
private:
  SceneContext *__ctx;
  SceneMaterial *__material;
  SceneGeometry *__geometry;

public:
  SceneObject(SceneContext *ctx)
      : __ctx(ctx), __material(new SceneMaterial),
        __geometry(new SceneGeometry) {
    this->setMaterial(__material);
    this->setFlag(QSGNode::OwnsMaterial);

    this->setGeometry(__geometry);
    this->setFlag(QSGNode::OwnsGeometry);
  }

  ~SceneObject() {
    this->setFlag(QSGNode::OwnsMaterial, false);
    delete __material;

    this->setFlag(QSGNode::OwnsGeometry, false);
    delete __geometry;
  }

  void updateState(const QRectF &rect, const float time) {
    this->__geometry->updateState(rect);
    this->markDirty(QSGNode::DirtyGeometry);

    this->__material->updateState(time);
    this->markDirty(QSGNode::DirtyMaterial);
  }
};

#endif // __SCENE_OBJECT_HPP
