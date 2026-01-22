fn sum(a: i32, b: i32) -> Result<String, String> {
    if a == 0 && b == 0 {
        return Err("division by zero".to_string());
    }
    Ok((a + b).to_string())
}

fn main() {
    let _x = 42;

    let _tupples: (char, i32) = ('a', 32);

    let (first, second) = (12, 17);

    println!("hey {:?}", sum(first, second))
}
// functions
