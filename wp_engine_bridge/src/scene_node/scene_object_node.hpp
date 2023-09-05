#ifndef __SCENE_OBJECT_NODE_HPP
#define __SCENE_OBJECT_NODE_HPP

#include <iostream>

#include <QtGui/QOpenGLBuffer>
#include <QtGui/QOpenGLFramebufferObject>
#include <QtGui/QOpenGLFunctions>
#include <QtGui/QOpenGLShaderProgram>
#include <QtQuick/QSGRenderNode>

#include "defer.hpp"
#include "scene_geometry.hpp"
#include "scene_material.hpp"

class SceneObjectNode : public QSGRenderNode {
public:
  explicit SceneObjectNode();
  ~SceneObjectNode();

  void render(const QSGRenderNode::RenderState *state) override;

  QRectF rect() const override;

  GLuint add_effect(const char *vertex_shader, const char *fragment_shader);

  void update_uniform(GLuint program_id, const char *name, const GlData &value);
  void update_geometry(const QRectF &rect);

  // StateFlags changedStates() const override {
  //   return StateFlags(BlendState);
  // }

  // RenderingFlags flags() const override {
  //   return BoundedRectRendering | DepthAwareRendering;
  // }

private:
  QRectF m_rect;

  QOpenGLBuffer m_vbo;
  QOpenGLShaderProgram m_program;

  std::vector<QOpenGLShaderProgram *> m_programs;
  std::map<GLuint, QOpenGLShaderProgram *> m_programs_qs; // quick search
  std::map<GLuint, std::map<std::string, GlData>> m_uniforms;

  float m_time = 1.0f;
};

SceneObjectNode::SceneObjectNode() {
  QOpenGLContext::currentContext()->functions()->initializeOpenGLFunctions();

  m_vbo.create();

  m_program.addShaderFromSourceCode(QOpenGLShader::Vertex,
                                    R"(
#version 330

layout(location = 0) in vec3 a_Position;
layout(location = 1) in vec2 a_TexCoord;

out vec2 texCoord;

uniform mat4 g_ModelViewProjectionMatrix;

void main() {
	texCoord = a_TexCoord;
	gl_Position = g_ModelViewProjectionMatrix * vec4(a_Position, 1.0);
}
            )");

  m_program.addShaderFromSourceCode(QOpenGLShader::Fragment,
                                    R"(
#version 330

in vec2 texCoord;

out vec4 outColor;

uniform sampler2D g_Texture0;

void main() {
    outColor = texture(g_Texture0, texCoord);
}
            )");

  m_program.link();
}

SceneObjectNode::~SceneObjectNode() {
  m_vbo.destroy();

  for (auto &program : this->m_programs) {
    delete program;
  }
}

void SceneObjectNode::render(const QSGRenderNode::RenderState *state) {
  auto *f = QOpenGLContext::currentContext()->functions();

  auto *projectionMatrix = state->projectionMatrix();
  // std::cout << "[C++]projectionMatrix: " << std::endl;
  // for (int i = 0; i < 4; ++i) {
  //   QVector4D row = projectionMatrix->row(i);
  //   std::cout << row.x() << " " << row.y() << " " << row.z() << " " <<
  //   row.w()
  //             << std::endl;
  // }

  // print m_rect
  std::cout << "[C++]m_rect: " << m_rect.right() << " " << m_rect.left() << " "
            << m_rect.top() << " " << m_rect.bottom() << std::endl;

  QOpenGLFramebufferObject fbo[] = {
      QOpenGLFramebufferObject{m_rect.size().toSize()},
      QOpenGLFramebufferObject{m_rect.size().toSize()}};

  auto *fbo_to_read = &fbo[0];
  auto *fbo_to_write = &fbo[1];

  bool first = true;
  for (auto *program : this->m_programs) {
    program->bind();
    defer(program->release());

    fbo_to_write->bind();
    defer(fbo_to_write->release());

    defer(std::swap(fbo_to_read, fbo_to_write));

    std::cout << "[C++]program id: " << program->programId()
              << " total: " << this->m_programs.size() << std::endl;

    program->setUniformValue("g_ModelViewProjectionMatrix", *projectionMatrix);
    program->setUniformValue("g_ModelViewProjectionMatrixInverse",
                             projectionMatrix->inverted());

    for (auto &[name_ref, value] : this->m_uniforms[program->programId()]) {
      const auto &name = name_ref;

      auto visitor = [&](auto &&value) {
        using T = std::decay_t<decltype(value)>;
        if constexpr (std::is_same_v<T, GLuint> || std::is_same_v<T, GLfloat> ||
                      std::is_same_v<T, QVector2D> ||
                      std::is_same_v<T, QVector3D> ||
                      std::is_same_v<T, QVector4D> ||
                      std::is_same_v<T, QMatrix2x2> ||
                      std::is_same_v<T, QMatrix3x3> ||
                      std::is_same_v<T, QMatrix4x4>) {
          program->setUniformValue(name.c_str(), value);
        } else if constexpr (std::is_same_v<T, Sampler2D>) {
          program->setUniformValue(name.c_str(), value.id);
          f->glActiveTexture(GL_TEXTURE0 + value.id);
          value.texture->bind();
        }
      };
      std::visit(visitor, value);
    }

    if (first) {
      first = false;
    } else {
      // reset to GL_TEXTURE0, the default
      f->glActiveTexture(GL_TEXTURE0);
      program->setUniformValue("g_Texture0", 0);
      f->glBindTexture(GL_TEXTURE_2D, fbo_to_read->texture());
    }

    m_vbo.bind();

    program->enableAttributeArray(0);
    program->enableAttributeArray(1);

    program->setAttributeBuffer("a_Position", GL_FLOAT, 0, 3,
                                5 * sizeof(float));
    program->setAttributeBuffer("a_TexCoord", GL_FLOAT, 3 * sizeof(float), 2,
                                5 * sizeof(float));

    f->glDrawArrays(GL_TRIANGLES, 0, 6);

    m_program.disableAttributeArray(0);
    m_program.disableAttributeArray(1);
  }

  {
    m_program.bind();
    defer(m_program.release());

    m_program.setUniformValue("g_ModelViewProjectionMatrix", *projectionMatrix);
    m_program.setUniformValue("g_ModelViewProjectionMatrixInverse",
                              projectionMatrix->inverted());
    m_program.setUniformValue("g_Texture0", 0);

    f->glActiveTexture(GL_TEXTURE0);
    f->glBindTexture(GL_TEXTURE_2D, fbo_to_read->texture());

    m_vbo.bind();
    m_program.enableAttributeArray(0);
    m_program.enableAttributeArray(1);
    m_program.setAttributeBuffer(0, GL_FLOAT, 0, 3, 5 * sizeof(float));
    m_program.setAttributeBuffer(1, GL_FLOAT, 3 * sizeof(float), 2,
                                 5 * sizeof(float));

    f->glDrawArrays(GL_TRIANGLES, 0, 6);

    m_program.disableAttributeArray(0);
    m_program.disableAttributeArray(1);
  }
}

QRectF SceneObjectNode::rect() const { return this->m_rect; }

void SceneObjectNode::update_geometry(const QRectF &rect) {
  static struct {
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
  } __attribute__((packed)) vertices[6];

  if (this->m_rect == rect) {
    return;
  }

  this->m_rect = rect;

  vertices[0].set(rect.right(), rect.top(), 0.0, 1.0, 0.0);
  vertices[1].set(rect.right(), rect.bottom(), 0.0, 1.0, 1.0);
  vertices[2].set(rect.left(), rect.top(), 0.0, 0.0, 0.0);

  vertices[3].set(rect.left(), rect.bottom(), 0.0, 0.0, 1.0);
  vertices[4].set(rect.left(), rect.top(), 0.0, 0.0, 0.0);
  vertices[5].set(rect.right(), rect.bottom(), 0.0, 1.0, 1.0);

  m_vbo.bind();
  m_vbo.allocate(vertices, sizeof(vertices));

  this->markDirty(QSGNode::DirtyGeometry);
}

GLuint SceneObjectNode::add_effect(const char *vertex_shader,
                                   const char *fragment_shader) {
  // hack for now
  if (this->m_programs.size() == 2) {
    return this->m_programs[0]->programId();
  }

  auto program = new QOpenGLShaderProgram{};
  program->addShaderFromSourceCode(QOpenGLShader::Vertex, vertex_shader);
  program->addShaderFromSourceCode(QOpenGLShader::Fragment, fragment_shader);
  program->link();

  this->m_programs.emplace_back(program);
  this->m_programs_qs.emplace(program->programId(), program);
  this->m_uniforms.emplace(program->programId(),
                           std::map<std::string, GlData>{});

  return program->programId();
}

void SceneObjectNode::update_uniform(GLuint program_id, const char *name,
                                     const GlData &value) {

  this->m_uniforms[program_id][name] = value;
  this->markDirty(QSGNode::DirtyMaterial);
}

#endif // __SCENE_OBJECT_NODE_HPP
