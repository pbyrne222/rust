// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn bar<T: Sized>() { }
fn foo<T: ?Sized>() { bar::<T>() } //~ ERROR the trait `core::marker::Sized` is not implemented
fn main() { }
