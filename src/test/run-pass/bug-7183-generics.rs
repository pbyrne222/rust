// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

trait Speak : Sized {
    fn say(&self, s:&str) -> String;
    fn hi(&self) -> String { hello(self) }
}

fn hello<S:Speak>(s:&S) -> String{
    s.say("hello")
}

impl Speak for int {
    fn say(&self, s:&str) -> String {
        format!("{}: {}", s, *self)
    }
}

impl<T: Speak> Speak for Option<T> {
    fn say(&self, s:&str) -> String {
        match *self {
            None => format!("{} - none", s),
            Some(ref x) => { format!("something!{}", x.say(s)) }
        }
    }
}


pub fn main() {
    assert_eq!(3i.hi(), "hello: 3".to_string());
    assert_eq!(Some(Some(3i)).hi(),
               "something!something!hello: 3".to_string());
    assert_eq!(None::<int>.hi(), "hello - none".to_string());

    assert_eq!(Some(None::<int>).hi(), "something!hello - none".to_string());
    assert_eq!(Some(3i).hi(), "something!hello: 3".to_string());
}
