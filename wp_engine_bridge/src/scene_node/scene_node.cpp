#include "scene_node.hpp"

constexpr const char *vertexShader = R"(
#version 330 core

layout (location = 0) in vec2 aPos;
layout (location = 1) in vec4 aColor;

out vec4 vertexColor;

uniform mat4 qt_Matrix;

void main() {
    gl_Position = qt_Matrix * vec4(aPos.x, aPos.y, 0.0, 1.0);
    vertexColor = aColor;
}


)";

constexpr const char *fragmentShader = R"(
#version 330 core

in vec4 vertexColor;

uniform float qt_Opacity;

out vec4 FragColor;

void main() {
  FragColor = qt_Opacity * vertexColor;
}

)";

SceneNode::SceneNode()
    : __material(new SceneMaterial(vertexShader, fragmentShader,
                                   QList<QByteArray>() << "aPos"
                                                       << "aColor")),
      __geometry(new SceneGeometry()) {}

SceneNode::~SceneNode() {
  // we shouldn't  delete __material and __geometry here
  // delete __material;
  // delete __geometry;
}

void SceneNode::updateState(const QRectF &rect, const QColor colors[3]) {
  this->setMaterial(__material);
  this->setFlag(QSGNode::OwnsMaterial);

  this->setGeometry(__geometry);
  this->setFlag(QSGNode::OwnsGeometry);

  this->__geometry->updateVertexData(rect, colors);
  this->markDirty(QSGNode::DirtyGeometry);
}
