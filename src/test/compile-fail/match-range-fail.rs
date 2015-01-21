// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//error-pattern: lower range bound
//error-pattern: only char and numeric types
//error-pattern: mismatched types

fn main() {
    match 5us {
      6us ... 1us => { }
      _ => { }
    };

    match "wow" {
      "bar" ... "foo" => { }
    };

    match 5us {
      'c' ... 100us => { }
      _ => { }
    };
}
