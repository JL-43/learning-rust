fn main() {

    //// Variable scope

    // s wont work here because it hasnt been declared
    {
        let s1 = "hello";
        println!("{s1}");
        // s will work here
    }
    // s wont work here because its now out of scope

    //// String Type

    // Typical Data Types covered before are typically stored in heap
    // the syntax below is a declaration of string (another type of string that is MUTABLE, which is important to show the ownership example)
    // normal string declaration is actually a string literal
    let mut s2 = String::from("hello");
    println!("{s2}");
    
    s2.push_str(", world");
    println!("{s2}");

    // this can be mutated, string literals CANNOT
    // we know the contents of string literals at compile time (this is why theyre fast and efficient, but their drawback is immutability)
    // with string, we can grow the text's size, we allocate an amount of memory from the heap, unknown at compile time, to hold the contents
        //1. The memory must be requested from the operating system at runtime
        //2. We need a way of returning this memory to the operating system when we're done with our String

        //1. is done by -> String::from -> allocates the memory
        //2. in other programming languages, this is achieved by using a Garbage Collector (GC) -> Rust uses the ownership concept
            //Rust takes a different path: the memory is automatically returned once the variable that owns it is out of scope.

    {
        let s3 = String::from("hello");
        // s3 is valid from this point forward
        println!("{s3}");
        let _s4 = s3;
        // println!("{s3}"); // this will not work because rust will protect you from using s3 that may be descoped after the s4 assignment
        // if a datatype does not have the "Copy" trait implemented, this error will occur
        // data types with static sizes (data types with the Copy trait) will be able to do this because their shallow and deep copies will be the same in size 
        // e.g. Integers, Bools, Characters, Floats, Tuples with the aforementioned types
    }
    // s3 is now out of scope--a special rust function called "drop" and s3's memory is returned

    // the copy concept applies to functions
    {
        let _s5 = String::from("hello");

        // takes_ownership(s);
        // s will no longer work after the function call

        let _x = 5;
        // makes_copy(x);
        // x will work here still
    }

    // because passing a value to a funciton means you also have to pass ownership, it gets difficult quickly
    // so instead we pass references to functions so we dont have to pass the ownership as well

    {
        // this is called BORROWING (since calculate_length doesnt have ownership)
        // BORROWED values cannot be mutated by default

        // let s1 = String::from("hello");
        // let len = calculate_length(&s1); // we pass a REFERENCE to s1
        

        // BORROWED values can be mutated by explicitly allowing it

        // let mut s = String::from("hello");
        // change(&mut s); // we pass the reference by using & and explicitly allow it to be mutable with mut

        // Mutable references have a limitation, you can only have 1 per scope
        // below will not work (this is called a data race)

        // let mut s = String::from("hello");

        // let r1 = &mut s;
        // let r2 = &mut s;
    }

}
