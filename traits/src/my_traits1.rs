pub trait Traits1 {
    fn return_num() -> i32;
    fn return_self() -> Self;
}

pub struct Str1 {pub x: i32, pub y: usize}
pub struct Str2 {pub a: usize}

impl Traits1 for Str1 {
    fn return_num() -> i32 {
        1
    }
    fn return_self() -> Self {
        Str1 {x:1, y:2}
    }
}

impl Traits1 for Str2 {
    fn return_num() -> i32 {
        2
    }
    fn return_self() -> Self {
        Self {a:3}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traits1_test() {
        let s1: Str1 = Str1{x:1, y:2};
        println!("{}", s1.x);
        assert_eq!(1, Str1::return_num());
        assert_eq!(1, Str1::return_self().x);

        let s2: Str2 = Str2{a:3};
        assert_eq!(2, Str2::return_num());
        assert_eq!(3, Str2::return_self().a);
    }
}