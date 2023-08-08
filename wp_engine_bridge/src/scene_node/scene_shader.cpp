#include "scene_shader.hpp"

#include <QtGui/QOpenGLContext>
#include <QtGui/QOpenGLFunctions>
#include <QtQuick/QQuickWindow>

#include "scene_material.hpp"

QQuickWindow *window = nullptr;

SceneShader::SceneShader(const char *vertexShader, const char *fragmentShader)
    : __vertexShader(vertexShader), __fragmentShader(fragmentShader),
      __g_Texture_id{0}, __g_TextureResolution_id{0}, __g_TextureRotation_id{0},
      __g_TextureTranslation_id{0}, __g_TextureMipMapInfo_id{0},
      __g_Texture_image{nullptr}, __g_Texture{nullptr} {

  __g_Texture_image[0] = new QImage{"/home/yuey/repos/wallpaper-engine-kde-rs/"
                                    "assets/wallpaper/materials/00009.png"};
  __g_Texture_image[1] = new QImage{
      "/home/yuey/repos/wallpaper-engine-kde-rs/"
      "assets/wallpaper/materials/masks/waterwaves_mask_c40ed93b.png"};
  __g_Texture_image[2] =
      new QImage{"/home/yuey/snap/steam/common/.local/share/Steam/steamapps/"
                 "common/wallpaper_engine/assets/materials/util/black.png"};

  __g_Texture[0] = window->createTextureFromImage(*__g_Texture_image[0]);
  __g_Texture[1] = window->createTextureFromImage(*__g_Texture_image[1]);
  __g_Texture[2] = window->createTextureFromImage(*__g_Texture_image[2]);
}

const char *SceneShader::vertexShader() const { return __vertexShader; }

const char *SceneShader::fragmentShader() const { return __fragmentShader; }

char const *const *SceneShader::attributeNames() const {
  static const char *const names[] = {"a_Position", "a_Color", "a_TexCoord", 0};
  return names;
}

void SceneShader::initialize() {
  QSGMaterialShader::initialize();

  for (size_t i = 0; i < g_TextureCount; i++) {
    __g_Texture_id[i] = program()->uniformLocation(g_Textures[i]);
    __g_TextureResolution_id[i] =
        program()->uniformLocation(g_TextureResolutions[i]);
    __g_TextureRotation_id[i] =
        program()->uniformLocation(g_TextureRotations[i]);
    __g_TextureTranslation_id[i] =
        program()->uniformLocation(g_TextureTranslations[i]);
    __g_TextureMipMapInfo_id[i] =
        program()->uniformLocation(g_TextureMipMapInfos[i]);
  }

  for (size_t i = 0; i < g_WPAttributeCount; i++) {
    __g_WPAttribute_id.insert(
        {g_WPAttributes[i], program()->uniformLocation(g_WPAttributes[i])});
  }
}

void SceneShader::updateState(const RenderState &state,
                              QSGMaterial *newMaterial,
                              QSGMaterial * /* oldMaterial */) {
  Q_ASSERT(program()->isLinked());

  auto *newM = (SceneMaterial *)newMaterial;

  if (state.isMatrixDirty()) {
    program()->setUniformValue(g_ModelViewProjectionMatrix,
                               state.combinedMatrix());
  }

  program()->setUniformValue(g_Time, newM->time());
  program()->setUniformValue(g_TextureResolutions[1], 2560.0f, 1440.0f, 2560.0f,
                             1440.0f);
  program()->setUniformValue(g_TextureResolutions[2], 2560.0f, 1440.0f, 2560.0f,
                             1440.0f);

  program()->setUniformValue("g_Point0", 0.0f, 0.0f);
  program()->setUniformValue("g_Point1", 1.0f, 0.0f);
  program()->setUniformValue("g_Point2", 1.0f, 1.0f);
  program()->setUniformValue("g_Point3", 0.0f, 1.0f);

  // from assets/wallpaper/scene.json
  program()->setUniformValue("g_Direction", 2.7933424f);
  program()->setUniformValue("g_Speed", 0.79000002f);
  program()->setUniformValue("g_Scale", 15.63f);
  program()->setUniformValue("g_Strength", 0.029999999f);
  program()->setUniformValue("g_Perspective", 0.003f);

  auto f = QOpenGLContext::currentContext()->functions();
  for (size_t i = 0; i < g_TextureCount; i++) {
    if (__g_Texture[i] == nullptr) {
      continue;
    }

    program()->setUniformValue(__g_Texture_id[i], (GLuint)i);
    f->glActiveTexture(GL_TEXTURE0 + i);
    __g_Texture[i]->bind();
  }

  // reset to GL_TEXTURE0, the default
  f->glActiveTexture(GL_TEXTURE0);
}