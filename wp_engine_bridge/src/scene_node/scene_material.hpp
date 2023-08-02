#ifndef __SCENE_MATERIAL_HPP
#define __SCENE_MATERIAL_HPP

#include <QtQuick/QSGMaterial>

#include "scene_shader.hpp"

class SceneMaterial : public QSGMaterial {
private:
  const QString __vertexShader;
  const QString __fragmentShader;
  const QList<QByteArray> __attributeNames;

public:
  SceneMaterial(const QString &vertexShader, const QString &fragmentShader,
                const QList<QByteArray> &attributeNames)
      : __vertexShader(vertexShader), __fragmentShader(fragmentShader),
        __attributeNames(attributeNames) {}

  QSGMaterialType *type() const override {
    static QSGMaterialType type;
    return &type;
  }

  QSGMaterialShader *createShader() const override {
    return new SceneShader(__vertexShader, __fragmentShader, __attributeNames);
  }
};

#endif // __SCENE_MATERIAL_HPP
