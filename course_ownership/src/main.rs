fn main() {
    // rules of ownership
    // 1. Each value in rust has a variable -> owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scope, the memory is freed

    let var = 1; // created in stack because it has a fixed size
    let mut s = "hello".to_string(); // created on the heap

    s.push_str(", world"); // you are able to add to the string because heap variables can change in size

    println!("{}", var);
    println!("{}", s);

    //// Move -> Moving ownership. Some types implement a move but some others implement a copy (See copy section)

    let x = vec!["tyler".to_string()];
    let y = x;
    // println!("{:?}", x); // doesnt work because the ownership was transferred to y
    println!("{:?}", y); // this will work because y owns the value

    //// Cloning -> If we want to get a value without taking ownership. Creates a "deep copy". May be expensive

    let a = vec!["tyler".to_string()];
    let b = a.clone();
    println!("{:?}", a);
    println!("{:?}", b); // both work because a deep copy was done. Both variables have ownership of their respective values

    //// Copy -> This works because integer data types implement copies (Recall that doing the same for a vector did not work. Vectors implement moves.)
    // If variable interacts with heap -> move ; If variable interacts with stack -> copy (JL Theory)
    let i = 1;
    let j = i;
    println!("i = {}, j = {}", i, j);

    //// More moves

    let s = String::from("takes"); // create a variable with a
    takes_ownership(s);
}

fn takes_ownership(s: String) {
    let some_string = s;

    println!("{}", some_string);
}

// both var and s are cleared from memory here since they are out of scope
