// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct P { child: Option<Box<P>> }
trait PTrait {
   fn getChildOption(&self) -> Option<Box<P>>;
}

impl PTrait for P {
   fn getChildOption(&self) -> Option<Box<P>> {
       static childVal: Box<P> = self.child.get();
       //~^ ERROR attempt to use a non-constant value in a constant
       panic!();
   }
}

fn main() {}
