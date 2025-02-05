mod hide {
    pub trait Super1{
        fn super1_method1(&self) ->i32 {
            1
        }
    }
    pub struct Token{}
}

pub trait Sub1: hide::Super1 {
    fn sub1_method1(&self) -> i32 {
        11
    }
}

pub trait Sub2 {
    fn sub2_method1(&self, _: hide::Token) -> i32{
        7
    }
    fn sub2_method2(&self) -> i32 {
        77
    }
}

pub struct StructMethod {
    pub x: i32,
}

impl hide::Super1 for StructMethod{}
impl Sub1 for StructMethod {
    //fn sub_method1(&self) ->i32 {}
}
impl Sub2 for StructMethod {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub1_test1() {
        let s1: StructMethod = StructMethod {x:1,};
        assert_eq!(11, s1.sub1_method1());
    }
    #[test]
    fn sub2_test1() {
        let s: StructMethod = StructMethod{x: 1};
        let t: hide::Token = hide::Token{};
        assert_eq!(7, s.sub2_method1(t));
    }
}