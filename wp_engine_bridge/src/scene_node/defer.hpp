#ifndef _DEFER_HPP_
#define _DEFER_HPP_

#define __MERGE(x, y) x_##y
#define _MERGE(x, y) __MERGE(x, y)
#define UNIQUE(x) _MERGE(x, __COUNTER__)

#define defer(expr) _defer_t UNIQUE(_defer) = [&]() { expr; }

template <class FUNC_T> struct _defer_t {
  FUNC_T func;
  _defer_t(FUNC_T f) : func(f) {}
  ~_defer_t() { func(); }
};

#endif // _DEFER_HPP_