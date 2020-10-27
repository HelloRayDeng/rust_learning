
fn main() {
    println!("Hello, world!");

    another_funtion(5, 6);

    another_function_a(7, 8);

    let x: i32 = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}
fn another_funtion(x: i32, y: i32) {
    println!("The value of x and y are: {} and {}", x, y);
}

fn another_function_a(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}