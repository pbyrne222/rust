// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-android: FIXME(#10381)
// min-lldb-version: 310

// compile-flags:-g

// === GDB TESTS ===================================================================================

// gdb-command:run

// gdb-command:print *a
// gdb-check:$1 = 1
// gdb-command:print *b
// gdb-check:$2 = {2, 3.5}


// === LLDB TESTS ==================================================================================

// lldb-command:run
// lldb-command:print *a
// lldb-check:[...]$0 = 1
// lldb-command:print *b
// lldb-check:[...]$1 = (2, 3.5)

#![allow(unused_variables)]
#![feature(box_syntax)]
#![omit_gdb_pretty_printer_section]

fn main() {
    let a = box 1i;
    let b = box() (2i, 3.5f64);

    zzz(); // #break
}

fn zzz() { () }
