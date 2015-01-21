// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// pp-exact

// Here we check that the parentheses around the body of `wsucc()` are
// preserved.  They are needed to disambiguate `{return n+1}; - 0` from
// `({return n+1}-0)`.

fn id<F>(f: F) -> int where F: Fn() -> int { f() }

fn wsucc(_n: int) -> int { id(|| { 1 }) - 0 }
fn main() { }
