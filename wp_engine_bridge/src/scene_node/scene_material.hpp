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
  SceneMaterial(const char *vertexShader, const char *fragmentShader)
      : __vertexShader(vertexShader), __fragmentShader(fragmentShader),
        __time(0.0) {}

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
