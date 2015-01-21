// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub mod kitties {

    pub struct cat<U> {
        info : Vec<U> ,
        meows : uint,

        pub how_hungry : int,
    }

    impl<U> cat<U> {
        pub fn speak<T>(&mut self, stuff: Vec<T> ) {
            self.meows += stuff.len();
        }

        pub fn meow_count(&mut self) -> uint { self.meows }
    }

    pub fn cat<U>(in_x : uint, in_y : int, in_info: Vec<U> ) -> cat<U> {
        cat {
            meows: in_x,
            how_hungry: in_y,
            info: in_info
        }
    }
}
