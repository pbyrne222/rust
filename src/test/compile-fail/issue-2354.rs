// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn foo() { //~ HELP did you mean to close this delimiter?
  match Some(x) {
      Some(y) { panic!(); }
      None    { panic!(); }
}

fn bar() {
    let mut i = 0;
    while (i < 1000) {}
}

fn main() {} //~ ERROR this file contains an un-closed delimiter
