// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// regression test for #8005

macro_rules! test { () => { fn foo() -> isize { 1is; } } }
                                             //~^ ERROR not all control paths return a value
                                             //~^^ HELP consider removing this semicolon

fn no_return() -> isize {} //~ ERROR  not all control paths return a value

fn bar(x: u32) -> u32 { //~ ERROR  not all control paths return a value
    x * 2; //~ HELP consider removing this semicolon
}

fn baz(x: u64) -> u32 { //~ ERROR  not all control paths return a value
    x * 2;
}

fn main() {
    test!();
}
