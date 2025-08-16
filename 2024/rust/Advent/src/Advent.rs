// use crate::d1;

fn a() -> i32 {
    1
}

#[cfg(test)]
mod tests {
    use  super::*;
    #[test]
    fn is_one() {
        assert_eq!(a(), 1)
    }
}
