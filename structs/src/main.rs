struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
// Unit-like structs
struct AlwaysEqual;

fn main() {

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };
    user1.email = String::from("anotheremail@example.com");
    let user2 = build_user(String::from("otheremail@example.com"), String::from("otherusername123"));

    let copy_user1 = User {
        active: user1.active,
        username: String::from("new_user"),
        email: user1.email,
        sign_in_count: user1.sign_in_count,
    };

    let copy_user2 = User {
        username: String::from("new_user"),
        ..user2
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}