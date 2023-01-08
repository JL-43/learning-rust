use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    loop{    
        println!("Please input your guess: ");

        // :: -> call the new() static method from the String class
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // old => crashes when input is not a number
        // let guess: u32 = guess.trim().parse()
        //     .expect("Please type a number!");

        // new => resets the game when input is not a number
        // switched from expect to match. Match returns Ok and Err for Good and Bad route respectively
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
