
fn multiply(x: &i32, y: &i32) -> i32 {
    x * y
}

fn main() {
    let a = 10;
    let b = 20;
    let result = multiply(&a, &b);
    println!("The multiplication of {} and {} is {}", a, b, result);
}
