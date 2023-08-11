#include "scene_shader.hpp"

#include <QtGui/QOpenGLContext>
#include <QtGui/QOpenGLFunctions>
#include <QtQuick/QQuickWindow>

#include "scene_material.hpp"

SceneShader::SceneShader(const char *vertexShader, const char *fragmentShader)
    : __vertexShader(vertexShader), __fragmentShader(fragmentShader) {}

const char *SceneShader::vertexShader() const { return __vertexShader; }

const char *SceneShader::fragmentShader() const { return __fragmentShader; }

const char *const *SceneShader::attributeNames() const {
  return ::attributeNames();
}

void SceneShader::initialize() { QSGMaterialShader::initialize(); }

void SceneShader::updateState(const RenderState &state,
                              QSGMaterial *newMaterial,
                              QSGMaterial * /* oldMaterial */) {
  Q_ASSERT(program()->isLinked());

  if (state.isMatrixDirty()) {
    program()->setUniformValue(g_ModelViewProjectionMatrix,
                               state.combinedMatrix());
    program()->setUniformValue(g_ModelViewProjectionMatrixInverse,
                               state.combinedMatrix().inverted());
  }

  auto *material = (SceneMaterial *)newMaterial;

  program()->setUniformValue(g_Time, material->time());

  for (auto &[nameRef, data] : material->g_WPCustomAttributes) {
    const auto &name = nameRef;
    std::visit(
        [&](auto &&value) {
          using T = std::decay_t<decltype(value)>;
          if constexpr (std::is_same_v<T, GLuint> ||
                        std::is_same_v<T, GLfloat> ||
                        std::is_same_v<T, QVector2D> ||
                        std::is_same_v<T, QVector3D> ||
                        std::is_same_v<T, QVector4D> ||
                        std::is_same_v<T, QMatrix2x2> ||
                        std::is_same_v<T, QMatrix3x3> ||
                        std::is_same_v<T, QMatrix4x4>) {
            program()->setUniformValue(name, value);
          }
        },
        data);
  }

  auto f = QOpenGLContext::currentContext()->functions();
  for (size_t i = 0; i < g_TextureCount; i++) {
    if (material->g_Texture[i] == nullptr) {
      continue;
    }

    program()->setUniformValue(g_Textures[i], (GLuint)i);
    program()->setUniformValue(
        g_TextureResolutions[i],
        QVector4D{(float)material->g_Texture_image[i]->width(),
                  (float)material->g_Texture_image[i]->height(),
                  (float)material->g_Texture_image[i]->width(),
                  (float)material->g_Texture_image[i]->height()});
    f->glActiveTexture(GL_TEXTURE0 + i);
    material->g_Texture[i]->bind();
  }

  // reset to GL_TEXTURE0, the default
  f->glActiveTexture(GL_TEXTURE0);
}