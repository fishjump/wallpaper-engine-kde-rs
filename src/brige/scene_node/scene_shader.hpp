#ifndef __SCENE_SHADER_HPP
#define __SCENE_SHADER_HPP

#include <QtQuick/QSGMaterialShader>

class SceneShader : public QSGMaterialShader {
private:
  const QString __vertexShader;
  const QString __fragmentShader;
  const QList<QByteArray> __attributeNames;

  int __qt_Matrix_id;
  int __qt_Opacity_id;

  // To have these two members, we can guarantee this class owns the data
  mutable QByteArray __attributeNameData;
  mutable QVector<const char *> __attributePointers;

public:
  SceneShader(const QString &vertexShader, const QString &fragmentShader,
              const QList<QByteArray> &attributeNames)
      : __vertexShader(vertexShader), __fragmentShader(fragmentShader),
        __attributeNames(attributeNames) {}

  const char *vertexShader() const override {
    return __vertexShader.toUtf8().constData();
  }

  const char *fragmentShader() const override {
    return __fragmentShader.toUtf8().constData();
  }

  char const *const *attributeNames() const override {
    // This segment of code is copied from qsgsimplematerial.h

    if (__attributeNameData.size()) {
      return __attributePointers.constData();
    }

    // Calculate the total number of bytes needed, so we don't get rellocs and
    // bad pointers while copying over the individual names.
    // Add an extra byte pr entry for the '\0' char.
    int total = 0;
    for (int i = 0; i < __attributeNames.size(); ++i) {
      total += __attributeNames.at(i).size() + 1;
    }
    __attributeNameData.reserve(total);

    // Copy over the names
    for (int i = 0; i < __attributeNames.size(); ++i) {
      __attributePointers.append(__attributeNameData.constData() +
                                 __attributeNameData.size());
      __attributeNameData.append(__attributeNames.at(i));
      __attributeNameData.append('\0');
    }

    // Append the "null" terminator
    __attributePointers.append(0);

    return __attributePointers.constData();
  }

  void initialize() override {
    QSGMaterialShader::initialize();
    __qt_Matrix_id = program()->uniformLocation("qt_Matrix");
    __qt_Opacity_id = program()->uniformLocation("qt_Opacity");
  }

  void updateState(const RenderState &state, QSGMaterial *newMaterial,
                   QSGMaterial *oldMaterial) override {
    Q_ASSERT(program()->isLinked());
    if (state.isMatrixDirty()) {
      program()->setUniformValue(__qt_Matrix_id, state.combinedMatrix());
    }
    if (state.isOpacityDirty()) {
      program()->setUniformValue(__qt_Opacity_id, state.opacity());
    }
  }
};

#endif // __SCENE_SHADER_HPP
