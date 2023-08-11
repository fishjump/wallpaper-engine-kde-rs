#ifndef __SCENE_SHADER_HPP
#define __SCENE_SHADER_HPP

#include <map>

#include <QtQuick/QSGGeometry>
#include <QtQuick/QSGMaterialShader>

#include "scene_context.hpp"
#include "texture_spec.hpp"

class SceneShader : public QSGMaterialShader {
private:
  const char *__vertexShader;
  const char *__fragmentShader;

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
