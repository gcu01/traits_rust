pub trait Trait2 {
    type MyType;
    fn set_me(arg: Self::MyType)-> Self;
}

pub struct Str3 {pub x: String, pub y: usize}

impl Trait2 for Str3 {
    type MyType = u32;
    fn set_me(arg: Self::MyType) -> Self{
        let a = arg + 1;
        Str3{x: format!("{}", a), y:5}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traits2_test1() {
        let s: Str3;
        s = Str3::set_me(1_u32);
        assert_eq!("2".to_string(), s.x);
    }
}