#ifndef __SCENE_MATERIAL_HPP
#define __SCENE_MATERIAL_HPP

#include <map>

#include <QtGui/QOpenGLContext>
#include <QtGui/QOpenGLFunctions>
#include <QtQuick/QSGGeometry>
#include <QtQuick/QSGMaterial>
#include <QtQuick/QSGMaterialShader>
#include <QtQuick/QSGTexture>

struct Sampler2D {
  QSGTexture *texture;
  GLuint id;
};

using GlData = std::variant<GLuint, GLfloat, QVector2D, QVector3D, QVector4D,
                            QMatrix2x2, QMatrix3x3, QMatrix4x4, Sampler2D>;

class SceneShader : public QSGMaterialShader {
private:
  const char *__vertexShader;
  const char *__fragmentShader;

public:
  SceneShader(const char *vertexShader, const char *fragmentShader);

  const char *vertexShader() const override;

  const char *fragmentShader() const override;

  char const *const *attributeNames() const override;

  void updateState(const RenderState &state, QSGMaterial *newMaterial,
                   QSGMaterial *oldMaterial) override;
};

class SceneMaterial : public QSGMaterial {
private:
  const char *m_vertexShader;
  const char *m_fragmentShader;

  std::map<const char *, GlData> m_uniforms;

public:
  SceneMaterial(const char *vertexShader, const char *fragmentShader)
      : m_vertexShader(vertexShader), m_fragmentShader(fragmentShader) {}

  QSGMaterialType *type() const override {
    static QSGMaterialType type;
    return &type;
  }

  QSGMaterialShader *createShader() const override {
    return new SceneShader{m_vertexShader, m_fragmentShader};
  }

  void update(const char *name, GlData data) { m_uniforms[name] = data; }

  const std::map<const char *, GlData> &uniforms() { return m_uniforms; }
};

SceneShader::SceneShader(const char *vertexShader, const char *fragmentShader)
    : __vertexShader(vertexShader), __fragmentShader(fragmentShader) {}

const char *SceneShader::vertexShader() const { return __vertexShader; }

const char *SceneShader::fragmentShader() const { return __fragmentShader; }

char const *const *SceneShader::attributeNames() const {
  static const char *const names[] = {"a_Position", "a_TexCoord", 0};
  return names;
}

void SceneShader::updateState(const RenderState &state,
                              QSGMaterial *newMaterial,
                              QSGMaterial *oldMaterial) {
  Q_ASSERT(program()->isLinked());

  if (state.isMatrixDirty()) {
    program()->setUniformValue("g_ModelViewProjectionMatrix",
                               state.combinedMatrix());
    program()->setUniformValue("g_ModelViewProjectionMatrixInverse",
                               state.combinedMatrix().inverted());
  }

  auto *material = (SceneMaterial *)newMaterial;
  const auto &uniforms = material->uniforms();

  auto f = QOpenGLContext::currentContext()->functions();

  for (auto &[name_ref, data] : uniforms) {
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
        program()->setUniformValue(name, value);
      } else if constexpr (std::is_same_v<T, Sampler2D>) {
        program()->setUniformValue(name, value.id);
        f->glActiveTexture(GL_TEXTURE0 + value.id);
        value.texture->bind();
      }
    };

    std::visit(visitor, data);
  }

  // reset to GL_TEXTURE0, the default
  f->glActiveTexture(GL_TEXTURE0);
}

#endif // __SCENE_MATERIAL_HPP
