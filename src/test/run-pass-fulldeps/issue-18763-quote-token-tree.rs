// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-android
// ignore-pretty: does not work well with `--test`

#![feature(quote)]

extern crate syntax;

use syntax::ext::base::ExtCtxt;

fn syntax_extension(cx: &ExtCtxt) {
    let _toks_1 = vec![quote_tokens!(cx, /** comment */ fn foo() {})];
    let name = quote_tokens!(cx, bar);
    let _toks_2 = vec![quote_item!(cx, static $name:int = 2;)];
    let _toks_4 = quote_tokens!(cx, $name:static $name:sizeof);
    let _toks_3 = vec![quote_item!(cx,
        /// comment
        fn foo() { let $name:int = 3; }
    )];
}

fn main() {
}
