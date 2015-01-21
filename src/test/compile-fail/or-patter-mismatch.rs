// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// error-pattern: mismatched types

enum blah { a(isize, isize, usize), b(isize, isize), }

fn main() { match blah::a(1, 1, 2us) { blah::a(_, x, y) | blah::b(x, y) => { } } }
