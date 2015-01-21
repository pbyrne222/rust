// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-pretty
// ignore-android

#![feature(quote)]

extern crate syntax;

use syntax::ext::base::ExtCtxt;

#[allow(dead_code)]
fn foobar(cx: &mut ExtCtxt) {
    quote_expr!(cx, 1i);
    quote_expr!(cx, 2i);
}

fn main() { }
