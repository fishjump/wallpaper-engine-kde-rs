#include "scene_shader.hpp"

#include <QtGui/QOpenGLContext>
#include <QtGui/QOpenGLFunctions>
#include <QtQuick/QQuickWindow>

#include "scene_material.hpp"

#include <iostream>

QQuickWindow *window = nullptr;

SceneShader::SceneShader(const QString &vertexShader,
                         const QString &fragmentShader)
    : __vertexShader(vertexShader), __fragmentShader(fragmentShader),
      __g_Texture0_image(new QImage{"assets/wallpaper/materials/00009.png"}),
      __g_Texture0(window->createTextureFromImage(*__g_Texture0_image)),
      __g_Texture1_image(({
        auto image = new QImage{200, 200, QImage::Format_RGB32};
        image->fill(QColor{32, 32, 32});
        image;
      })),
      __g_Texture1(window->createTextureFromImage(*__g_Texture1_image)) {}

const char *SceneShader::vertexShader() const {
  return __vertexShader.toUtf8().constData();
}

const char *SceneShader::fragmentShader() const {
  return __fragmentShader.toUtf8().constData();
}

char const *const *SceneShader::attributeNames() const {
  static const char *const names[] = {"aPos", "aColor", "aTexCoord", 0};
  return names;
}

void SceneShader::initialize() {
  QSGMaterialShader::initialize();
  __qt_Matrix_id = program()->uniformLocation("qt_Matrix");
  __qt_Opacity_id = program()->uniformLocation("qt_Opacity");
  __g_Texture0_id = program()->uniformLocation("g_Texture0");
  __g_Texture1_id = program()->uniformLocation("g_Texture1");
}

void SceneShader::updateState(const RenderState &state,
                              QSGMaterial *newMaterial,
                              QSGMaterial * /* oldMaterial */) {
  Q_ASSERT(program()->isLinked());

  if (state.isMatrixDirty()) {
    program()->setUniformValue(__qt_Matrix_id, state.combinedMatrix());
  }

  if (state.isOpacityDirty()) {
    program()->setUniformValue(__qt_Opacity_id, state.opacity());
  }

  program()->setUniformValue(__g_Texture0_id, 0);
  program()->setUniformValue(__g_Texture1_id, 1);

  glActiveTexture(GL_TEXTURE0);
  __g_Texture0->bind();

  glActiveTexture(GL_TEXTURE1);
  __g_Texture1->bind();
}