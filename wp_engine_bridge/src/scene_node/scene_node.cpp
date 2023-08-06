#include "scene_node.hpp"

constexpr const char *vertexShader = R"(
#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aTexCoord;

out vec3 vertexColor;
out vec2 texCoord;

uniform mat4 qt_Matrix;

void main() {
    gl_Position = qt_Matrix * vec4(aPos.x, aPos.y, 0.0, 1.0);
    vertexColor = aColor;
    texCoord = aTexCoord;
}

)";

constexpr const char *fragmentShader = R"(
#version 330 core

in vec3 vertexColor;
in vec2 texCoord;

uniform float qt_Opacity;
uniform sampler2D g_Texture0;
uniform sampler2D g_Texture1;

out vec4 FragColor;

void main() {
  FragColor = qt_Opacity * texture(g_Texture1, texCoord) * texture(g_Texture0, texCoord);
}

)";

SceneNode::SceneNode()
    : __material(new SceneMaterial(vertexShader, fragmentShader)),
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

void SceneNode::setTexture(QSGTexture *texture) {
  this->markDirty(QSGNode::DirtyMaterial);
}