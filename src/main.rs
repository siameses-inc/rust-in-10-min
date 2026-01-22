fn sum(a: i32, b: i32) -> Result<i32, &'static str> {
    if a == 0 && b == 0 {
        return Err("zero numbers not allowed");
    }
    Ok(a + b)
}

fn main() {
    let _x = 42;

    let _tupples: (char, i32) = ('a', 32);

    let (first, second) = (12, 17);

    println!("hey {:?}", sum(first, second))
}

#[cfg(test)]
mod tests {
    use super::sum;

    #[test]
    fn sum_zero_numbers() {
        let result = sum(0, 0);

        match result {
            Ok(s) => assert_eq!(s, 0),
            Err(e) => eprintln!("{}", e),
        }
    }

    #[test]
    fn error_on_summing_zeros() {
        let result = sum(2, 5);

        match result {
            Ok(s) => assert_eq!(s, 7),
            _ => panic!(""),
        }
    }
}
