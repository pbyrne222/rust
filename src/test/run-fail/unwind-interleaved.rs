// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// error-pattern:fail

fn a() { }

fn b() { panic!(); }

fn main() {
    let _x = vec!(0i);
    a();
    let _y = vec!(0i);
    b();
}
