fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;

    let spaces = "     ";
    let spaces = spaces.len();
    println!("The number of spaces is: {}", spaces);

    let x = MAX_POINTS;
    println!("The value of x is: {}", x);

    let tup: (i32, f64, u8) = (500, 6.4, 1); 
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of tup 1: {}", five_hundred);
    println!("The value of tup 2: {}", six_point_four);
    println!("The value of tup 3: {}", one);

    let t = true;
    let f: bool = false;
    println!("The value of t is: {}", t);
    println!("The value of f is: {}", f);

    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3, 5];
    let c: usize = 10;
    //println!("The value of a[0] is: {}", a[c]);

    //let a = [1, 2, 3, 4, 5];
    //let index = 10;
    //let element = a[index];
    //println!("The value of element is: {}", element);

}
