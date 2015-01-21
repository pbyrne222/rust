// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn f() { }
struct S(Box<FnMut()>); //~ ERROR explicit lifetime bound required
pub static C: S = S(f);


fn g() { }
type T = Box<FnMut()>;  //~ ERROR explicit lifetime bound required
pub static D: T = g;

fn main() {}
