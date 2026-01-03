fn main() {
    let mut users_list: Vec<User> = Vec::new();
    users_list.push(User {
        UserName: String::from("sampleUser"),
        Age: 23,
    });
    for user in &users_list {
        println!("User {} and Age is: {}", user.UserName, user.Age);
    }
}

struct User {
    UserName: String,
    Age: u16,
}
