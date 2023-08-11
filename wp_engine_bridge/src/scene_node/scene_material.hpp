#ifndef __SCENE_MATERIAL_HPP
#define __SCENE_MATERIAL_HPP

#include <QtQuick/QSGMaterial>
#include <QtQuick/QSGTexture>

#include "scene_shader.hpp"

class SceneMaterial : public QSGMaterial {
private:
  const char *__vertexShader;
  const char *__fragmentShader;

  float __time;

public:
  using GlData = std::variant<GLuint, GLfloat, QVector2D, QVector3D, QVector4D,
                              QMatrix2x2, QMatrix3x3, QMatrix4x4>;

  int g_Texture_id[g_TextureCount];
  int g_TextureResolution_id[g_TextureCount];
  int g_TextureRotation_id[g_TextureCount];
  int g_TextureTranslation_id[g_TextureCount];
  int g_TextureMipMapInfo_id[g_TextureCount];
  QImage *g_Texture_image[g_TextureCount];
  QSGTexture *g_Texture[g_TextureCount];

  std::map<const char *, GLuint> g_WPAttribute_id;
  std::map<const char *, GlData> g_WPCustomAttributes;

  SceneMaterial()
      : __vertexShader(VERTEX_SHADER), __fragmentShader(FRAGMENT_SHADER),
        __time(0.0), g_Texture_id{0}, g_TextureResolution_id{0},
        g_TextureRotation_id{0}, g_TextureTranslation_id{0},
        g_TextureMipMapInfo_id{0}, g_Texture_image{nullptr},
        g_Texture{nullptr} {
    g_Texture_image[0] = new QImage{"/home/yuey/repos/wallpaper-engine-kde-rs/"
                                    "assets/wallpaper/materials/00009.png"};
    g_Texture_image[1] = new QImage{
        "/home/yuey/repos/wallpaper-engine-kde-rs/"
        "assets/wallpaper/materials/masks/waterwaves_mask_c40ed93b.png"};
    g_Texture_image[2] =
        new QImage{"/home/yuey/snap/steam/common/.local/share/Steam/steamapps/"
                   "common/wallpaper_engine/assets/materials/util/black.png"};

    g_Texture[0] = window->createTextureFromImage(*g_Texture_image[0]);
    g_Texture[1] = window->createTextureFromImage(*g_Texture_image[1]);
    g_Texture[2] = window->createTextureFromImage(*g_Texture_image[2]);

    g_WPCustomAttributes.emplace("g_Point0", QVector2D{0.0, 0.0});
    g_WPCustomAttributes.emplace("g_Point1", QVector2D{1.0, 0.0});
    g_WPCustomAttributes.emplace("g_Point2", QVector2D{1.0, 1.0});
    g_WPCustomAttributes.emplace("g_Point3", QVector2D{0.0, 1.0});

    // from assets/wallpaper/scene.json
    g_WPCustomAttributes.emplace("g_Direction", 2.7933424f);
    g_WPCustomAttributes.emplace("g_Speed", 0.79000002f);
    g_WPCustomAttributes.emplace("g_Scale", 15.63f);
    g_WPCustomAttributes.emplace("g_Strength", 0.029999999f);
    g_WPCustomAttributes.emplace("g_Perspective", 0.003f);
  }

  QSGMaterialType *type() const override {
    static QSGMaterialType type;
    return &type;
  }

  QSGMaterialShader *createShader() const override {
    return new SceneShader(__vertexShader, __fragmentShader);
  }

  void updateState(float time) { __time = time; }

  float time() { return __time; }
};

#endif // __SCENE_MATERIAL_HPP
