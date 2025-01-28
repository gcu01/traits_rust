
pub trait Traits3<'a, T> {
    fn func1(self, arg: T) -> Self;
    fn func2(self, arg: &'a Self) -> Self;
    fn func3(self, arg: &'a T) -> Self;
}

struct Str4 {x: i32}

impl<'a> Traits3<'a, i32> for Str4 {
    fn func1(self, arg: i32)-> Str4 {
        Str4{x: arg}
    }
    fn func2(self, arg: &'a Self) -> Str4{
        Str4{x: arg.x}
    }
    fn func3(self, arg: &'a i32)-> Str4 {
        Str4{x: *arg}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traits3_test1() {
        let mut s: Str4 = Str4{x:1};
        let s2: Str4 = Str4{x:2};
        s = Str4::func1(s, 4);
        assert_eq!(4, s.x);
        s = s.func2(&s2);
        assert_eq!(2, s.x);
        s = s.func3(&3);
        assert_eq!(3, s.x);
    }
}