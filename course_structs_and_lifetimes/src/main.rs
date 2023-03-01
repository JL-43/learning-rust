// //// Structs
// // Let you name and package together multiple values into single values so you can interact with them with that one value
// // 3 types of struct:
// // 1. Named field (gives name to each component)
// // 2. Tuple-like (component is identified in the order that they appear)
// // 3. Unit-like (no components)

// // named field struct
// struct User {
//     active: bool,
//     username: String,
//     sign_in_count: u32,
// }
// // tuple-like struct
// struct Coordinates(i32, i32, i32);

// // unit-like struct
// struct UnitStruct;

// fn main() {
//     // Named field struct: initializing values
//     let user1 = User {
//         active: true,
//         username: String::from("JL"),
//         sign_in_count: 0,
//     };
//     // Named field struct: accessing values
//     println!(
//         "Named field struct of user1: {0}, {1}, {2}",
//         user1.active, user1.username, user1.sign_in_count
//     );

//     let user2 = build_user(String::from("JL again"));
//     println!(
//         "Named field struct of user2: {0}, {1}, {2}",
//         user2.active, user2.username, user2.sign_in_count
//     );

//     // Tuple-like struct: initializing values
//     let coords = Coordinates(1, 2, 3);
//     // Tuple-like struct: accessing values
//     println!(
//         "Tuple-like struct: {0}, {1}, {2}",
//         coords.0, coords.1, coords.2
//     )

//     // Unit-like struct: //A little later on in this course in the "traits" section
// }

// // function that builds/initializes the named field struct
// fn build_user(username: String) -> User {
//     User {
//         username, //username parameter = username attribute are mapped automatically since they are named same
//         active: true, //preset values for the rest so that they dont have to be initalized every time
//         sign_in_count: 1,
//     }
// }

////////////////////////////

//// Methods -> functions defined in the context of a struct, enum, or trait object.
/// Always has "self" as first parameter. "self" references the object that is calling the method

// struct Square {
//     width: u32,
//     height: u32,
// }

// // impl -> implementation === method
// impl Square {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn whats_my_width(&self) -> u32 {
//         self.width
//     }

//     // for mutable structs
//     fn change_width(&mut self, new_width: u32) {
//         self.width = new_width;
//     }
// }
// fn main() {
//     // let sq = Square {
//     //     width: 5,
//     //     height: 5,
//     // };

//     // println!("{0}", sq.area());
//     // println!("{0}", sq.whats_my_width());

//     //// mutable struct
//     let mut sq = Square {
//         width: 5,
//         height: 5,
//     };

//     println!("{0}", sq.area());
//     println!("{0}", sq.whats_my_width());
//     sq.change_width(10); // its not really a square anymore though hehe
//     println!("{0}", sq.whats_my_width());
// }

////////////////////////////

/// Lifetimes -> every reference has a lifetime. Every lifetime is implicit and inferred. One of Rust's most distinctive features
// Prevents dangling references
// fn main() {
//     let r;

//     {
//         let x = 5;
//         r = &x
//     } // x is dropped

//     println!("{0}", r); // borrowed value does not live long enough
// }

// fn main() {
//     // EXPLICIT lifetime
//     // 'a <- explicit lifetime keyword (a is common practice)
//     // &i32 // normal referenced i32
//     // &'a i32 // explicit lifetime referenced i32
//     // &'a mut i32 // explicit lifetime referenced mutable i32
//     let r;

//     {
//         let x = 5;
//         r = &x
//     }

//     println!("{0}", r);
// }

// // this guy's explanation of lifetime is pretty abhorrent

// fn example<'a>(x: &'a str) -> &'a str {
//     x
// }
