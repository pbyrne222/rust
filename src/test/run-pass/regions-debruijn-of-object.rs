// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct ctxt<'tcx> {
    x: &'tcx i32
}

trait AstConv<'tcx> {
    fn tcx<'a>(&'a self) -> &'a ctxt<'tcx>;
}

fn foo(conv: &AstConv) { }

fn bar<'tcx>(conv: &AstConv<'tcx>) {
    foo(conv)
}

fn main() { }
