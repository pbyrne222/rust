// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-tidy-linelength
// aux-build:coherence-orphan-lib.rs

#![feature(optin_builtin_traits)]

extern crate "coherence-orphan-lib" as lib;

use lib::TheTrait;

struct TheType;

impl TheTrait<usize> for isize { } //~ ERROR E0117

impl TheTrait<TheType> for isize { } //~ ERROR E0117

impl TheTrait<isize> for TheType { }

impl !Send for Vec<isize> { } //~ ERROR E0117
//~^ ERROR conflicting

fn main() { }
