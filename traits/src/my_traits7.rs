pub trait T {
    fn method1(&self) -> u64;
    fn method2(&self) -> i32;
}

struct SS1 {x :u64,}
struct SS2 {a: i32}

impl T for SS1 {
    fn method1(&self) -> u64 {
        641
    }
    fn method2(&self) -> i32 {
        321
    }
}

impl T for SS2{
    fn method1(&self) -> u64 {
        642
    }
    fn method2(&self) -> i32 {
        322
    }
}

pub fn f1(q: &dyn T) -> u64 {
    q.method1()
}

pub fn f2(w: Box<&dyn T>) -> i32 {
    w.method2()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ss1: SS1 = SS1{x:1};
        let ss2: SS2 = SS2{a:2};
        assert_eq!(641, ss1.method1());
        assert_eq!(641, f1(&ss1));
        assert_eq!(322, f2(Box::new(&ss2)));
    }
}
