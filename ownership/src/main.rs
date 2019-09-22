struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// "tuple structs"
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = build_user(String::from("poops"), String::from("peepee"));

    // Struct update syntax
    let user2 = User {
        username: String::from("I have to take a BIG dump this time around"),
        email: String::from("bigdumper@gmail.com"),
        ..user1
    };

    print_user(&user1);
    print_user(&user2);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 0,
        active: true,
    }
}

fn print_user(user: &User) {
    println!("{} {} {} {}", user.username, user.email, user.sign_in_count, user.active);
}
