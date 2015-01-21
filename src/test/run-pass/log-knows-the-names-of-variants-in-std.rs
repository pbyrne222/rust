// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[derive(Clone, Show)]
enum foo {
  a(uint),
  b(String),
}

fn check_log<T: std::fmt::Show>(exp: String, v: T) {
    assert_eq!(exp, format!("{:?}", v));
}

pub fn main() {
    let mut x = Some(foo::a(22u));
    let exp = "Some(a(22u))".to_string();
    let act = format!("{:?}", x);
    assert_eq!(act, exp);
    check_log(exp, x);

    x = None;
    let exp = "None".to_string();
    let act = format!("{:?}", x);
    assert_eq!(act, exp);
    check_log(exp, x);
}
