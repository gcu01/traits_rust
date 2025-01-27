pub trait Supertrait {
    fn super_method(&self) -> bool {
        dbg!("calling from super_method");
        true
    }
}

pub trait Subtrait {
    fn sub_method(&self) -> bool{
        dbg!("calling from sub_method");
        true
    }
}
#[derive(Debug)]
pub struct StrS1 {pub x: i32}
//#[derive(Debug)]
//pub struct StrS2 {pub x: i32}

impl Supertrait for StrS1 {
    fn super_method(&self) -> bool{
        dbg!("StrS1 super_method");
        if self.x == 2 {
            self.sub_method();
        }
        true
    }
}

impl Subtrait for StrS1 {
    fn sub_method(&self) -> bool {
        dbg!("Strs1 sub_method");
        if self.x == 1 {
            self.super_method();
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let s1: StrS1 = StrS1{x: 1};
        assert!(s1.sub_method());
        assert!(s1.super_method());
    }
}