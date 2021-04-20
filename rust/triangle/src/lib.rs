use std::cmp::PartialOrd;
use std::ops::Add;

pub struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}

impl<T> Triangle<T>
where
    T: PartialEq + PartialOrd + Add<Output = T> + From<u8> + Copy,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let sides_len_ok = sides.iter().all(|&s| s > 0_u8.into());
        let sides_sum_ok = sides[0] + sides[1] >= sides[2]
            && sides[1] + sides[2] >= sides[0]
            && sides[2] + sides[0] >= sides[1];
        match (sides_len_ok, sides_sum_ok) {
            (true, true) => Some(Triangle {
                a: sides[0],
                b: sides[1],
                c: sides[2],
            }),
            _ => None,
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.b != self.c && self.c != self.a
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.b == self.c || self.c == self.a
    }
}
