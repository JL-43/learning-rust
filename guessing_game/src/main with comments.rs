use rand::Rng;

use std::cmp::Ordering;
use std::io;


fn main() {
    // to loop the guessing game

    println!("Guess the number!");

    // generate a secret number with the random number generator library initialized above (use rand::Rng)
    // start..=end is how to define a range. INCLUSIVE of start and end
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    loop{
        // initialize the "guess" variable and let it be a mutable variable -> declare is as a new instance of a string
        let mut guess = String::new();
    
        // takes the user input and enters it into the string we pass to it (advanced: this also returns an enum called "Result")
            // Result can be either Ok and Err
        // expect -> called when the Err variant of the Result enum is generated
        // without expect, the program will still compile but Rust will warn you that you do not have a way to catch erroneous values
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // "Shadows" the previous value of guess with a new one -> converts guess into a numerical value so the values are compared numerically (u32 => unsigned 32-bit integer)
        // trim() to remove the /n (newline) that is automatically generated by .read_line()
        // parse() convert to another type (argument of the type to parse into is in the variable declaration: u32)
        // expect() if input cannot be parsed into u32
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        //v2 of above to continue when guess is not parse-able
        // switch from expect to match => move from crashing on an error to handling an error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, 
            // _ is a catch all value, in this case, we want to match all Err values. 
            // continue sends the program to go to the next NEW iteration of the loop
        };
    
        // Use the cmp::Ordering library to "compare" the guess to the secret number; (guess.cmp(&secret_numer) => guess compare to secret number)
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    
        println!("You guessed: {guess}");
    }
}
