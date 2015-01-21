// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:use_from_trait_xc.rs

extern crate use_from_trait_xc;
pub use use_from_trait_xc::Trait;

fn main() {
    match () {
        Trait { x: 42us } => () //~ ERROR use of trait `Trait` in a struct pattern
    }
}
