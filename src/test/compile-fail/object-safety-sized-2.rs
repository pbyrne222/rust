// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Check that we correctly prevent users from making trait objects
// from traits where `Self : Sized`.

trait Bar
    where Self : Sized
{
    fn bar<T>(&self, t: T);
}

fn make_bar<T:Bar>(t: &T) -> &Bar {
    t
        //~^ ERROR `Bar` is not object-safe
        //~| NOTE the trait cannot require that `Self : Sized`
}

fn make_bar_explicit<T:Bar>(t: &T) -> &Bar {
    t as &Bar
        //~^ ERROR `Bar` is not object-safe
        //~| NOTE the trait cannot require that `Self : Sized`
}

fn main() {
}
