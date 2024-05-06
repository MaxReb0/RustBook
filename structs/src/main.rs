
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;


fn main() {
    let mut user1 = build_user(String::from("exampleemail@example.com"), String::from("test_username"));
    user1.email = String::from("anotheremail@example.com");

    // println!("Here is the email: {}", user1.email);

    //Struct update syntax

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // is the same as: 

    let user2 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("This should be zero: {}", black.0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
