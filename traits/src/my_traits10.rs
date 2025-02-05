//mod my_traits9;
use crate::my_traits9::{self, *};
pub struct DownStream {x: i32,}
pub fn dwn_str1(a: impl my_traits9::Sub1) -> i32 {
    a.sub1_method1()
}
//sealed trait
/* 
impl my_traits9::Sub for DownStream {
    fn sub_method1(&self) -> i32 {
        6
    }
}

impl my_traits9::hide::Super{
    fn super_method1(&self) -> i32 {
        5
    }
}*/

impl my_traits9::Sub2 for DownStream {
    /* 
    fn sub2_method1(&self, _: my_traits9::hide::Token) -> i32 {
        1
    }*/
    fn sub2_method2(&self) -> i32 {
        55
    }
}

fn dwn_str2(a: impl my_traits9::Sub2) -> i32 {
    //let t: my_traits9::hide::Token = my_traits9::hide::Token{};
    //a.sub2_method1(t);
    a.sub2_method2()
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test1() {
        let one: StructMethod = StructMethod{x:12,};
        assert_eq!(11, one.sub1_method1());
    }
    #[test]
    fn test2() {
        let par:StructMethod = StructMethod{x:5,};
        assert_eq!(11, dwn_str1(par));
    }
    #[test]
    fn test3() {
        let d: DownStream = DownStream { x: 1 };
        assert_eq!(55, d.sub2_method2());
    }
}