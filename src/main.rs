fn main() {
    let user = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("User: {}", user.username)
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
