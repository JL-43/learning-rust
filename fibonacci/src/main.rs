// Input  : n = 2
// Output : 1

// Input  : n = 9
// Output : 34

// below code overflows at n=48
use std::io;

fn fib(n: u32) -> u32{
    if n <= 1{
        return n;
    }
    return fib(n - 1) + fib(n - 2);
    
}

fn main() {
    println!("Please input your fibonacci number: ");

    // :: -> call the new() static method from the String class
    let mut fibonacci_number = String::new();

    io::stdin().read_line(&mut fibonacci_number)
        .expect("Failed to read line");

    let fibonacci_number: u32 = match fibonacci_number.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let result = fib(fibonacci_number);

    println!("{result}");
}
