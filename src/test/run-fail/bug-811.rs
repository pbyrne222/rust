// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// error-pattern:quux
fn test00_start(ch: chan_t<int>, message: int) { send(ch, message); }

type task_id = int;
type port_id = int;

struct chan_t<T> {
    task: task_id,
    port: port_id,
}

fn send<T:Send>(_ch: chan_t<T>, _data: T) { panic!(); }

fn main() { panic!("quux"); }
