pub fn add_one(number: i32) -> i32 {
    number + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(3);
        let expected = 4;
        assert_eq!(result, expected);
    }
}
