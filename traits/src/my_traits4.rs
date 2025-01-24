use std::process::Output;

pub trait Add {
    type Rhs;
    type Output;
    fn add(self, rhs: Self::Rhs) -> Self::Output;
}

pub trait AddGen<Rhs, Output> {
    fn add_gen(self, rhs: Rhs) -> Output;
}

pub struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Rhs = Point;
    type Output = Point;
    fn add(self, rhs: Self::Rhs) -> Self::Output {
        Point{x:self.x+rhs.x, y:self.y + rhs.y}
    }
}

impl AddGen<i32, Point> for Point {
    fn add_gen(self, rhs: i32) -> Point {
        Point{x: self.x+rhs, y: self.y+rhs}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test1() {
        let s1: Point = Point{x:1, y:1};
        let s2: Point = Point { x: 2, y: 2 };
        assert_eq!(3, s1.add(s2).x);
    }

    #[test]
    fn add_test2() {
        let s1: Point = Point{x:1, y:1};
        let s3: i32 = 3;
        assert_eq!(4, s1.add_gen(s3).x);
    }
}