// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.



mod foo {
    pub fn x() -> int { return 1; }
}

mod bar {
    pub fn y() -> int { return 1; }
}

pub fn main() { foo::x(); bar::y(); }
