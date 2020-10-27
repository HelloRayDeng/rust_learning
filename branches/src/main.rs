fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    let condition = true;
    let number = if condition { 5 } else  { "six" };
    
    println!("The value of number is: {}", number);
}
