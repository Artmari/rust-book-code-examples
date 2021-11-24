// STRUCTS
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// Tuple Struct
// struct Color(i32, i32, i32);
// struct Point(i32, i32);

// Unit like structs
// struct AlwaysEqual

pub fn users_struct() {
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    fn build_user(username: String, email: String) -> User {
        User {
            username,
            email,
            sign_in_count: 1,
            active: true,
        }
    }
    let mut user_1 = User {
        email: String::from("fhfhfhfhfh@mail.com"),
        username: String::from("maria"),
        active: true,
        sign_in_count: 1,
    };
    user_1.username = String::from("new user name");

    let user_2 = build_user(String::from("Tom"), String::from("dhdhd@mail.com"));

    println!("Username: {}", user_1.username);
    println!("Added new user: {:#?}", user_2);
}
