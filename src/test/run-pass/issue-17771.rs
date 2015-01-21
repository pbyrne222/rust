// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

trait Aaa {}

impl<'a> Aaa for &'a mut (Aaa + 'a) {}

struct Bar<'a> {
    writer: &'a mut (Aaa + 'a),
}

fn baz(_: &mut Aaa) {
}

fn foo<'a>(mut bar: Bar<'a>) {
    baz(&mut bar.writer);
}

fn main() {
}
