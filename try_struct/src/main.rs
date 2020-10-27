fn main() {
    println!("Hello, world!");
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someuername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = build_user(user1.email, user1.username);
    //let user3 = build_user(user1.email, user1.username);

    println!("user1: {}, {}, {}, {}", user2.email, user2.username, user1.active, user1.sign_in_count);

    user2.email = String::from("hello@example.com");
    user2.username = String::from("hello");

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername"),
        ..user2
    };
    user2.email = user3.email;
    user2.username = user2.username;
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}