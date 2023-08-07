#ifndef __SCENE_SHADER_HPP
#define __SCENE_SHADER_HPP

#include <QtQuick/QSGGeometry>
#include <QtQuick/QSGMaterialShader>

#include "texture_spec.hpp"

__attribute__((packed)) struct ColoredPoint2DWithTexCoord {
  float x;
  float y;
  float z;

  float r;
  float g;
  float b;

  float u;
  float v;

  void set(float x, float y, float z, float r, float g, float b, float u,
           float v) {
    this->x = x;
    this->y = y;
    this->z = z;

    this->r = r;
    this->g = g;
    this->b = b;

    this->u = u;
    this->v = v;
  }
};

QSGGeometry::Attribute attributes[] = {
    QSGGeometry::Attribute::create(0, 3, GL_FLOAT, true),  // a_Position
    QSGGeometry::Attribute::create(1, 3, GL_FLOAT, false), // a_Color
    QSGGeometry::Attribute::create(2, 2, GL_FLOAT, false)  // a_TexCoord
};

QSGGeometry::AttributeSet AttrColoredPoint2DWithTexCoord = {
    3, sizeof(ColoredPoint2DWithTexCoord), attributes};

class SceneShader : public QSGMaterialShader {
private:
  const char *__vertexShader;
  const char *__fragmentShader;

  int __g_Texture_id[g_TextureCount];
  int __g_TextureResolution_id[g_TextureCount];
  int __g_TextureRotation_id[g_TextureCount];
  int __g_TextureTranslation_id[g_TextureCount];
  int __g_TextureMipMapInfo_id[g_TextureCount];
  QImage *__g_Texture_image[g_TextureCount];
  QSGTexture *__g_Texture[g_TextureCount];

  std::map<const char *, GLuint> __g_WPAttribute_id;

public:
  SceneShader(const char *vertexShader, const char *fragmentShader);

  const char *vertexShader() const override;

  const char *fragmentShader() const override;

  char const *const *attributeNames() const override;

  void initialize() override;

  void updateState(const RenderState &state, QSGMaterial *newMaterial,
                   QSGMaterial *oldMaterial) override;
};

#endif // __SCENE_SHADER_HPP
