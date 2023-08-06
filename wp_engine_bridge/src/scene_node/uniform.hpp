#ifndef __UNIFORM_HPP
#define __UNIFORM_HPP

#include <string_view>

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

#define X(name) std::string_view name{#name};
TEX_NAME_LIST
#undef X

#define X(name) std::string_view name##Resolution{#name "Resolution"};
TEX_NAME_LIST
#undef X

#endif // __UNIFORM_HPP
