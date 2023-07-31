#include <QtQuick/QSGBasicGeometryNode>
#include <QtQuick/QSGGeometry>
#include <QtQuick/QSGSimpleMaterialShader>

struct MaterialData {
  float r, g, b, a;
};

class CustomMaterialShader : public QSGSimpleMaterialShader<MaterialData> {
  QSG_DECLARE_SIMPLE_SHADER(CustomMaterialShader, MaterialData);

public:
  const char *vertexShader() const override {
    return "attribute highp vec2 vertex;               \n"
           "uniform highp mat4 qt_Matrix;              \n"
           "void main() {                              \n"
           "  gl_Position = qt_Matrix * vec4(vertex.x, vertex.y, 0.0, 1.0);\n"
           "}";
  }

  const char *fragmentShader() const override {
    return "uniform lowp float qt_Opacity;             \n"
           "uniform lowp vec4 color;                   \n"
           "void main() {                              \n"
           "  gl_FragColor = qt_Opacity * vec4(1.0, 0.0, 0.0, 1.0);     \n"
           "}";
  }

  QList<QByteArray> attributes() const override {
    return QList<QByteArray>() << "vertex";
  }

  void updateState(const MaterialData *newData,
                   const MaterialData * /*unused: _oldData*/) {
    program()->setUniformValue("color", newData->r, newData->g, newData->b,
                               newData->a);
  }
};

QSGGeometry *createQSGGeometry() {
  auto *geometry = new QSGGeometry(QSGGeometry::defaultAttributes_Point2D(), 3);
  geometry->setDrawingMode(GL_TRIANGLES);
  return geometry;
}

void updatePosition(QSGGeometry *geometry, float x1, float y1, float x2,
                    float y2, float x3, float y3) {
  auto *vertices = geometry->vertexDataAsPoint2D();
  vertices[0].set(x1, y1);
  vertices[1].set(x2, y2);
  vertices[2].set(x3, y3);
}
