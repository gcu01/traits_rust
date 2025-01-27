mod my_traits1;
use my_traits1::*;
mod my_traits2;
mod my_traits3;
mod my_traits4;
mod my_traits5;
mod my_traits6;
fn main() {
    let s: Str1 = Str1{x:1, y:10};
    let tostr1: String = s.to_string();
    let tostr2 = ToString::to_string(&s);

    println!("{} {}", tostr1, tostr2);
}
