// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[derive(Eq, PartialEq, PartialOrd, Ord)]
enum Test<'a> {
    Int(&'a int),
    Slice(&'a [u8]),
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct Version {
    vendor_info: &'static str
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct Foo(&'static str);

fn main() {}
