// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:plugin_crate_outlive_expansion_phase.rs
// ignore-stage1

#![feature(plugin)]

#[plugin] #[no_link]
extern crate plugin_crate_outlive_expansion_phase;

pub fn main() {}
