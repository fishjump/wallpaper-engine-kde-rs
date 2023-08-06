#ifndef __SCENE_SHADER_HPP
#define __SCENE_SHADER_HPP

#include <QtQuick/QSGGeometry>
#include <QtQuick/QSGMaterialShader>

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
    QSGGeometry::Attribute::create(0, 3, GL_FLOAT, true),  // aPos
    QSGGeometry::Attribute::create(1, 3, GL_FLOAT, false), // aColor
    QSGGeometry::Attribute::create(2, 2, GL_FLOAT, false)  // aTexCoord
};

QSGGeometry::AttributeSet AttrColoredPoint2DWithTexCoord = {
    3, sizeof(ColoredPoint2DWithTexCoord), attributes};

class SceneShader : public QSGMaterialShader {
private:
  const QString __vertexShader;
  const QString __fragmentShader;
  const QList<QByteArray> __attributeNames;

  int __qt_Matrix_id;
  int __qt_Opacity_id;
  int __g_Texture0_id;
  int __g_Texture1_id;

  QImage *__g_Texture0_image;
  QSGTexture *__g_Texture0;

  QImage *__g_Texture1_image;
  QSGTexture *__g_Texture1;

  // To have these two members, we can guarantee this class owns the data
  mutable QByteArray __attributeNameData;
  mutable QVector<const char *> __attributePointers;

public:
  SceneShader(const QString &vertexShader, const QString &fragmentShader);

  const char *vertexShader() const override;

  const char *fragmentShader() const override;

  char const *const *attributeNames() const override;

  void initialize() override;

  void updateState(const RenderState &state, QSGMaterial *newMaterial,
                   QSGMaterial *oldMaterial) override;
};

#endif // __SCENE_SHADER_HPP
