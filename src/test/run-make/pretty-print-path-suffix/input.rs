// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_type="lib"]

pub fn
foo() -> i32
{ 45 }

pub fn bar() -> &'static str { "i am not a foo." }

pub mod nest {
    pub fn foo() -> &'static str { "i am a foo." }

    struct S;
    impl S {
        fn foo_method(&self) -> &'static str {
            return "i am very similiar to foo.";
        }
    }
}
