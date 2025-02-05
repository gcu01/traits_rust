pub trait Default {
    fn default() -> Self;
}

pub trait SubDefault: Default {
    fn subdefault() -> Self;
}

impl Default for u8 {
    fn default() -> Self {
        10
    }
}

pub struct Color{
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct SubStr8 {
    pub x1: i32,
    pub y1: usize,
}

impl Default for SubStr8{
    fn default() -> Self {
        SubStr8{ x1: 1001, y1: 110}
    }
}
impl SubDefault for SubStr8 {

    fn subdefault() -> Self {
        SubStr8 { x1:101, y1: 100 }
    }
}

#[cfg(test)]
mod tests{
    use std::ops::Sub;

    use crate::my_traits8;

    use super::*;

    #[test]
    fn default_test1() {
        let c1: Color = Color{r: <u8 as my_traits8::Default>::default(), g: <u8 as std::default::Default>::default(), b: 1,};
        assert_eq!(10, c1.r);
        assert_eq!( 0, c1.g);
    }

    #[test]
    fn default_test2() {
        let s: SubStr8;
        assert_eq!(SubStr8::default().x1, 1001);
        assert_eq!(SubStr8::subdefault().y1, 100);
    }
}