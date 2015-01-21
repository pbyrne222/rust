// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// error-pattern:aFdEfSeVEE

/* We're testing that link_args are indeed passed when nolink is specified.
So we try to compile with junk link_args and make sure they are visible in
the compiler output. */

#![feature(link_args)]

#[link_args = "aFdEfSeVEEE"]
extern {}

fn main() { }
