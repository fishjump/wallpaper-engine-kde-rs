#ifndef __SCENE_MATERIAL_HPP
#define __SCENE_MATERIAL_HPP

#include <map>

#include <QtGui/QOpenGLContext>
#include <QtGui/QOpenGLFunctions>
#include <QtQuick/QSGGeometry>
#include <QtQuick/QSGMaterial>
#include <QtQuick/QSGMaterialShader>
#include <QtQuick/QSGTexture>

/**
 * @brief Represents a 2D sampler for textures.
 *
 * Holds both a pointer to the texture and its associated ID.
 */
struct Sampler2D {
  QSGTexture *texture;
  GLuint id;
};

/**
 * @brief Variant type to encapsulate different OpenGL data types.
 */
using GlData = std::variant<GLuint, GLfloat, QVector2D, QVector3D, QVector4D,
                            QMatrix2x2, QMatrix3x3, QMatrix4x4, Sampler2D>;

/**
 * @brief A custom material shader class for scene rendering.
 */
class SceneShader : public QSGMaterialShader {
public:
  explicit SceneShader(const char *vertex_shader, const char *fragment_shader);

  const char *vertexShader() const override;
  const char *fragmentShader() const override;
  const char *const *attributeNames() const override;
  void updateState(const RenderState &state, QSGMaterial *new_material,
                   QSGMaterial *old_material) override;

private:
  const char *m_vertex_shader;
  const char *m_fragment_shader;
};

/**
 * @brief A material class to manage shaders and uniform configurations.
 */
class SceneMaterial : public QSGMaterial {
public:
  explicit SceneMaterial(const char *vertex_shader,
                         const char *fragment_shader);

  QSGMaterialType *type() const override;
  QSGMaterialShader *createShader() const override;

  void update(const char *name, GlData data);

  const std::map<const char *, GlData> &uniforms();

private:
  const char *m_vertex_shader;
  const char *m_fragment_shader;
  std::map<const char *, GlData> m_uniforms;
};

// ******************************
// * SceneShader implementation *
// ******************************
SceneShader::SceneShader(const char *vertex_shader, const char *fragment_shader)
    : m_vertex_shader(vertex_shader), m_fragment_shader(fragment_shader) {}

const char *SceneShader::vertexShader() const { return m_vertex_shader; }

const char *SceneShader::fragmentShader() const { return m_fragment_shader; }

char const *const *SceneShader::attributeNames() const {
  static const char *const names[] = {"a_Position", "a_TexCoord", 0};
  return names;
}

void SceneShader::updateState(const RenderState &state,
                              QSGMaterial *new_material,
                              QSGMaterial * /* old_material */) {
  Q_ASSERT(program()->isLinked());

  if (state.isMatrixDirty()) {
    program()->setUniformValue("g_ModelViewProjectionMatrix",
                               state.combinedMatrix());
    program()->setUniformValue("g_ModelViewProjectionMatrixInverse",
                               state.combinedMatrix().inverted());
  }

  auto *material = (SceneMaterial *)new_material;
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

// ********************************
// * SceneMaterial implementation *
// ********************************
SceneMaterial::SceneMaterial(const char *vertex_shader,
                             const char *fragment_shader)
    : m_vertex_shader(vertex_shader), m_fragment_shader(fragment_shader) {}

QSGMaterialType *SceneMaterial::type() const {
  static QSGMaterialType type;
  return &type;
}

QSGMaterialShader *SceneMaterial::createShader() const {
  return new SceneShader{m_vertex_shader, m_fragment_shader};
}

void SceneMaterial::update(const char *name, GlData data) {
  m_uniforms[name] = data;
}

const std::map<const char *, GlData> &SceneMaterial::uniforms() {
  return m_uniforms;
}

#endif // __SCENE_MATERIAL_HPP
