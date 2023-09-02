#ifndef __SCENE_OBJECT_HPP
#define __SCENE_OBJECT_HPP

#include <QtQuick/QSGGeometryNode>

#include "scene_geometry.hpp"
#include "scene_material.hpp"

class SceneObject : public QSGGeometryNode {
public:
  explicit SceneObject();
  ~SceneObject();

  bool initialize();

  void set_vertex_shader(const char *vertex_shader);

  void set_fragment_shader(const char *fragment_shader);

  void update_uniform(const char *name, const GlData &value);

  void update_geometry(const QRectF &rect);

private:
  const char *m_vertex_shader;
  const char *m_fragment_shader;

  SceneMaterial *m_material;
  SceneGeometry *m_geometry;
};

SceneObject::SceneObject() {}

SceneObject::~SceneObject() {
  this->setFlag(QSGNode::OwnsMaterial, false);
  if (m_material != nullptr) {
    delete m_material;
  }

  this->setFlag(QSGNode::OwnsGeometry, false);
  if (m_geometry != nullptr) {
    delete m_geometry;
  }
}

bool SceneObject::initialize() {
  if (m_vertex_shader == nullptr) {
    return false;
  }

  if (m_fragment_shader == nullptr) {
    return false;
  }

  if (m_vertex_shader != nullptr) {
    m_material = new SceneMaterial{m_vertex_shader, m_fragment_shader};
    this->setMaterial(m_material);
    this->setFlag(QSGNode::OwnsMaterial);
  }

  if (m_fragment_shader != nullptr) {
    m_geometry = new SceneGeometry{};
    this->setGeometry(m_geometry);
    this->setFlag(QSGNode::OwnsGeometry);
  }

  return true;
}

void SceneObject::set_vertex_shader(const char *vertex_shader) {
  m_vertex_shader = vertex_shader;
}

void SceneObject::set_fragment_shader(const char *fragment_shader) {
  m_fragment_shader = fragment_shader;
}

void SceneObject::update_uniform(const char *name, const GlData &value) {
  if (m_material == nullptr) {
    return;
  }

  m_material->update(name, value);
  this->markDirty(QSGNode::DirtyMaterial);
}

void SceneObject::update_geometry(const QRectF &rect) {
  if (m_geometry == nullptr) {
    return;
  }

  m_geometry->update(rect);
  this->markDirty(QSGNode::DirtyGeometry);
}

#endif // __SCENE_OBJECT_HPP
