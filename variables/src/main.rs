fn main() {

    //// Variables and Mutability
    println!("----- Variables and Mutability -----");

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of ix is: {x}");

    let y = 5;
    println!("The value of y is: {y}");
    let y = y + 1; 
    // the first value is over-"shadowed" by the second. 
    // meaning the compiler will see the second value (or whatever the last value is)
    // shadowing is different from mutable values
    // we are creating a "new variable" with shadowing
        {
            // When the scope of the "shadow" is over, it returns to its previous value
            let y = y * 2;
            println!("The value of y is: {y}")
        }
    println!("The value of y is: {y}");

    // since we are creating a new variable everytime we use the let keyword
    // we can change the type of the value but retain the same name

    // this works!

    let spaces = "   ";
    let spaces = spaces.len();

    println!("{spaces}");

    //this doesnt! gives mismatch type error

    // let mut spaces2 = "   ";
    // spaces2 = spaces2.len();

    // println!("{spaces2}");

    //// Data Types
    println!("----- Data Types -----");

    // this will work
    let guess: u32 = "42".parse().expect("Not a number: ");
    println!("{guess}");

    // but this will not
    // it will require type annotations for this case
    // let guess2 = "42".parse().expect("Not a number: ");
    // println!("{guess2}")

    // other types:
    // scalar => represent a single value => four primary scalar types: integers, floating-point numbers, Booleans, and characters
        // integers => number without a fractional component (u32 is an example, though are many more)
        // 
}
