use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Data {
    v: i32,
}

impl Add<i32> for Data {
    type Output = Data;

    fn add(self, rhs: i32) -> Self::Output {
        return Data { v: self.v + rhs };
    }
}

impl Add for Data {
    type Output = Data;

    fn add(self, rhs: Self) -> Self::Output {
        return Data { v: self.v + rhs.v };
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn add_int() {
        let dt = Data { v: 5 };
        assert_eq!(Data { v: 7 }, dt + 2);
    }

    #[test]
    fn add_self() {
        let d1 = Data { v: 5 };
        let d2 = Data { v: 2 };
        assert_eq!(Data { v: 7 }, d1 + d2);
    }
}
