use std::ops::Add;

pub fn add<T>(a: T, b: T) -> T::Output
    where T: Add
{
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
}
