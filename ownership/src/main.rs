fn main() {
    //let s1 = String::from("hello");
    //let s2 = s1;

    //println!("{}, world!", s1); // move
    //let s2 = s1.clone();
    //println!("s1 = {}, s2 = {}", s1, s2);
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    //let y = s;

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward
    //let z = x;

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    let len = calculate_length_ref(&s2);

    println!("The length of '{}' is {}.", s2, len);

    let mut s = String::from("hello");
    change(&mut s);

    {
        let r1 = &mut s;
    }
    let r2 = &mut s; // error
    //println!("{}, {}", r1, r2);

    let my_string = String::from("helloworldhello ");
    let word = first_word(&my_string[..]);
    let word = first_word(&my_string);
    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[..]);
    let word = first_word(my_string_literal);
    //s.clear();
    let second = second_word(&my_string);

    println!("the first word is: {}", word);
    println!("the second word is:{},", second);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{}, {}", slice[0], slice[1]);

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s:String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn change (some_string: &mut String) {
    some_string.push_str(", world");
}

//Test
//fn dangle() -> &String {
//    let s = String::from("hello");
//
//    &s
//}

fn first_word(s:&str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s:&str) -> &str {

    let bytes = s.as_bytes();
    let mut first_found = false;
    let mut j = 0;

    for (i, &item) in bytes.iter().enumerate() {
        
        if item == b' ' {
            if !first_found {
                first_found = true;
                j = i + 1;
                continue;
            }
            return &s[j..i];
        }
    }

    if first_found {
        if j != bytes.len(){
            return &s[j..];
        }
    }

    &s[..]
}