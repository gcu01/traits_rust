mod my_traits1;
use my_traits1::*;

fn main() {
    let s: Str1 = Str1{x:1, y:10};
    let tostr1: String = s.to_string();
    let tostr2 = ToString::to_string(&s);

    println!("{} {}", tostr1, tostr2);
}
