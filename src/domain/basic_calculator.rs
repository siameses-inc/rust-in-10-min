pub trait Calculator {
    fn add(&self, a: i32, b: i32) -> i32;
    fn sub(&self, a: i32, b: i32) -> i32;
    fn mul(&self, a: i32, b: i32) -> i32;
    fn div(&self, a: i32, b: i32) -> i32;
}

pub struct BasicCalculator;

impl Calculator for BasicCalculator {
    fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
    fn sub(&self, a: i32, b: i32) -> i32 {
        a - b
    }
    fn mul(&self, a: i32, b: i32) -> i32 {
        a * b
    }
    fn div(&self, a: i32, b: i32) -> i32 {
        a / b
    }
}
