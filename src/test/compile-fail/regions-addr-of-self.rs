// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct dog {
    cats_chased: usize,
}

impl dog {
    pub fn chase_cat(&mut self) {
        let p: &'static mut usize = &mut self.cats_chased; //~ ERROR cannot infer
        *p += 1us;
    }

    pub fn chase_cat_2(&mut self) {
        let p: &mut usize = &mut self.cats_chased;
        *p += 1us;
    }
}

fn dog() -> dog {
    dog {
        cats_chased: 0us
    }
}

fn main() {
    let mut d = dog();
    d.chase_cat();
    println!("cats_chased: {}", d.cats_chased);
}
