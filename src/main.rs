mod domain;

use crate::domain::basic_calculator::{BasicCalculator, Calculator};

fn main() {
    let _x = 42;

    let _tupples: (char, i32) = ('a', 32);

    let (first, second) = (12, 17);

    let calc = BasicCalculator;
    println!("add {:?}", calc.add(first, second));
    println!("sub {:?}", calc.sub(first, second));
    println!("mul {:?}", calc.mul(first, second));
    println!("div {:?}", calc.div(first, second));
}

#[cfg(test)]
mod tests {
    use super::{BasicCalculator, Calculator};

    #[test]
    fn adding() {
        let calc = BasicCalculator;
        let result = calc.add(2, 5);

        assert_eq!(result, 7)
    }

    #[test]
    fn subtracting() {
        let calc = BasicCalculator;
        let result = calc.sub(2, 5);

        assert_eq!(result, -3)
    }

    #[test]
    fn multiplying() {
        let calc = BasicCalculator;
        let result = calc.mul(2, 5);

        assert_eq!(result, 10)
    }

    #[test]
    fn div() {
        let calc = BasicCalculator;
        let result = calc.div(10, 5);

        assert_eq!(result, 2)
    }

    #[test]
    fn basic_summing() {
        let calc = BasicCalculator;
        let result = calc.add(2, 5);

        assert_eq!(result, 7)
    }
}
