// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:issue-13560-1.rs
// aux-build:issue-13560-2.rs
// aux-build:issue-13560-3.rs
// ignore-stage1

// Regression test for issue #13560, the test itself is all in the dependent
// libraries. The fail which previously failed to compile is the one numbered 3.

extern crate "issue-13560-2" as t2;
extern crate "issue-13560-3" as t3;

fn main() {}
