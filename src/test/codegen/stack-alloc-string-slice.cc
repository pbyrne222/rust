// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#include <stddef.h>

struct slice {
  char const *p;
  size_t len;
};

extern "C"
void test() {
  struct slice s = { .p = "hello",
                     .len = 5 };
}
