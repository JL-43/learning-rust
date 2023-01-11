fn main() {

    // structs are like tuples but each data has a name (key-value pair)
    // similar to objects in OOP but with data attributes only (no methods)

    //// defining a struct

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    //// instantiating a struct

    // notice that we assigned a value to email before username. Because they are key-value pairs, the order of assignment does not matter
    let user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // call a value from the struct with the dot notation

    // println!("{user1.username}"); // this syntax doesnt work with dot notation, maybe have to pass it to a variable?

    // let user1_username = user1.username;
    // println!("{user1_username}");

    // or just use the other syntax for inserting variables into strings

    println!("Username: {0}, Email: {1}, Active: {2}, Sign-in Count: {3}", user1.username, user1.email, user1.active, user1.sign_in_count);

    //// we can also create a mutable instance of the user

    let mut user2 = User{
        email: String::from("someotherperson@example.com"),
        username: String::from("someotherusername123"),
        active: true,
        sign_in_count: 5, 
    };

    println!("User2 Email before mutation: {0}", user2.email);

    user2.email = String::from("anotheremail@example.com");

    println!("User2 Email after mutation: {0}", user2.email);

    //// we can also instantiate structs as a return of functions

    // can do it like this (where email: email)
    // fn build_user(email: String, username: String) -> User{
    //     User{
    //         email: email,
    //         username: username,
    //         active: true,
    //         sign_in_count: 1,
    //     }
    // }

    // or like this, since the args have the same name as the struct attributes
        fn build_user(email: String, username: String) -> User{
        User{
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    // we have to use to_string() to change from a &str to an owned String. Struct instances should own their data by default
    let user3 = build_user("emailemail@email.com".to_string(), "anotherusernameagain".to_string());

    println!("User3 Email: {0}", user3.email);

    //// Struct update syntax

    // Set the value of user2 but copy the rest of the values of user1

    let user2 = User {
        email: String::from("another@example.com"),
        username: String ::from("anotherusername567"),
        ..user1 // copies the other values of user 1 into user2
    };

    println!("Username: {0}, Email: {1}, Active: {2}, Sign-in Count: {3}", user2.username, user2.email, user2.active, user2.sign_in_count);

    //// Using tuple structs without named fields to create different types

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // black and origin are different types despite having the same struct definition
}
