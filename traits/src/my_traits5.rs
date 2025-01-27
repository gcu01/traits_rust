// implementation of trait methods
pub trait Supertrait {
    fn method(&self) -> i32{
        println!("in SUPER-trait");
        1
    }
}

pub trait Subtrait: Supertrait {
    fn method(&self) -> i32 {
        println!("in SUB-trait");
        2
    }
}

pub struct StrTrait{pub x: i32,}
pub struct StrTrait1{pub x: i32,}

impl Supertrait for StrTrait {}
// not working because Supertrait is not implemented for StrTrait1
//impl Subtrait for StrTrait1 {}
impl Subtrait for StrTrait {}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn supertrait_test() {
        let s1: StrTrait = StrTrait{x: 10};
        assert_eq!(1, <StrTrait as Supertrait>::method(&s1));
        assert_eq!(2, <StrTrait as Subtrait>::method(&s1));
    }

}