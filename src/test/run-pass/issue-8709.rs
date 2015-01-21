// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

macro_rules! sty {
    ($t:ty) => (stringify!($t))
}

macro_rules! spath {
    ($t:path) => (stringify!($t))
}

fn main() {
    assert_eq!(sty!(int), "int");
    assert_eq!(spath!(std::option), "std::option");
}
