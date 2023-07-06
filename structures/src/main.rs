fn main() {
    let chandram = User {
        name: "Chandram".to_string(),
        email: "chandram@chandramdutta.me".to_string(),
        age: 19,
        active: true,
    };
    let user2 = User {
        email: String::from("another@example.com"),
        ..chandram
    };
    let black = Color(0, 0, 0);
}

struct User {
    name: String,
    email: String,
    age: usize,
    active: bool,
}

struct Color(i32, i32, i32);
