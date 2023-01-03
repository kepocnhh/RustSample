fn main() {
    let src = build_user(
        String::from("someone@example.com"),
        String::from("someusername123")
    );
    let u0 = User {
        active: src.active,
        username: String::from("u0username"),
        email: String::from("u0@example.com"),
        sign_in_count: src.sign_in_count,
    };
    println!("User: {}", u0.username);
    let u1 = User {
        username: String::from("u1username"),
        email: String::from("u1@example.com"),
        ..u0
    };
    println!("User: {}", u1.username);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
