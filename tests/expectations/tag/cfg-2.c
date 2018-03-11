#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

#if (defined(NOT_DEFINED) || defined(DEFINED))
struct Foo {
  int32_t x;
};
#endif

#if defined(NOT_DEFINED)
struct Bar {
  struct Foo y;
};
#endif

#if defined(DEFINED)
struct Bar {
  struct Foo z;
};
#endif

struct Root {
  struct Bar w;
};

void root(struct Root a);
