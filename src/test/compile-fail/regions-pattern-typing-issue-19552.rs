// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn assert_send<T: Send>(_t: T) {}

fn main() {
    let line = String::new();
    match [line.as_slice()] { //~ ERROR `line` does not live long enough
        [ word ] => { assert_send(word); }
    }
}
