#ifndef __UNIFORM_HPP
#define __UNIFORM_HPP

#include <cstddef>

#define TEX_NAME_LIST                                                          \
  X(g_Texture0)                                                                \
  X(g_Texture1)                                                                \
  X(g_Texture2)                                                                \
  X(g_Texture3)                                                                \
  X(g_Texture4)                                                                \
  X(g_Texture5)                                                                \
  X(g_Texture6)                                                                \
  X(g_Texture7)                                                                \
  X(g_Texture8)                                                                \
  X(g_Texture9)                                                                \
  X(g_Texture10)                                                               \
  X(g_Texture11)                                                               \
  X(g_Texture12)

constexpr const char *g_Textures[] = {
#define X(name) #name,
    TEX_NAME_LIST
#undef X
};

constexpr size_t g_TextureCount = sizeof(g_Textures) / sizeof(const char *);

constexpr const char *g_TextureResolutions[] = {
#define X(name) #name "Resolution",
    TEX_NAME_LIST
#undef X
};

constexpr const char *g_TextureRotations[] = {
#define X(name) #name "Rotation",
    TEX_NAME_LIST
#undef X
};

constexpr const char *g_TextureTranslations[] = {
#define X(name) #name "Translation",
    TEX_NAME_LIST
#undef X
};

constexpr const char *g_TextureMipMapInfos[] = {
#define X(name) #name "MipMapInfo",
    TEX_NAME_LIST
#undef X
};

#define WP_ATTRIBUTE_LIST                                                      \
  X(g_ModelMatrix)                                                             \
  X(g_ViewProjectionMatrix)                                                    \
  X(g_ModelViewProjectionMatrix)                                               \
  X(g_AltModelMatrix)                                                          \
  X(g_ModelMatrixInverse)                                                      \
  X(g_ModelViewProjectionMatrixInverse)                                        \
  X(g_EffectTextureProjectionMatrix)                                           \
  X(g_EffectTextureProjectionMatrixInverse)                                    \
  X(g_LightsPosition)                                                          \
  X(g_LightsColorPremultiplied)                                                \
  X(g_Time)                                                                    \
  X(g_DayTime)                                                                 \
  X(g_PointerPosition)                                                         \
  X(g_TexelSize)                                                               \
  X(g_TexelSizeHalf)                                                           \
  X(g_Bones)                                                                   \
  X(g_Screen)                                                                  \
  X(g_ParallaxPosition)

#define X(name) constexpr const char *name = #name;
WP_ATTRIBUTE_LIST
#undef X

constexpr const char *const g_WPAttributes[] = {
#define X(name) #name,
    WP_ATTRIBUTE_LIST
#undef X
};

constexpr size_t g_WPAttributeCount =
    sizeof(g_WPAttributes) / sizeof(const char *);

#endif // __UNIFORM_HPP