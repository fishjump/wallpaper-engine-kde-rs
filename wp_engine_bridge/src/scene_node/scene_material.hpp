#ifndef __SCENE_MATERIAL_HPP
#define __SCENE_MATERIAL_HPP

#include <QtQuick/QSGMaterial>
#include <QtQuick/QSGTexture>

#include "scene_shader.hpp"

class SceneMaterial : public QSGMaterial {
private:
  const QString __vertexShader;
  const QString __fragmentShader;

public:
  SceneMaterial(const QString &vertexShader, const QString &fragmentShader)
      : __vertexShader(vertexShader), __fragmentShader(fragmentShader) {}

  QSGMaterialType *type() const override {
    static QSGMaterialType type;
    return &type;
  }

  QSGMaterialShader *createShader() const override {
    return new SceneShader(__vertexShader, __fragmentShader);
  }
};

#endif // __SCENE_MATERIAL_HPP
