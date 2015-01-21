// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_type = "lib"]
#![feature(globs)]

mod m {
    pub use self::a::Foo;

    mod a {
        pub struct Foo;
    }

    mod b {
        pub use super::*;
    }
}
