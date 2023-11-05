use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {

        println!("Please input your guess.");

        let mut guess = String::new(); //variables are immutable (const) by default | data types have 'associated functions' (methods). 
    
            // in summary the above line is simply assigning an empty mutable string to guess
    
        io::stdin()                                 //call standard input from io library
            .read_line(&mut guess)              //expect input and assign it to the argued variable, guess. This is done directly without creating copies using & and this reference is mutable? not 100%. 
                                             // i think &mut means go straight to the guess variable in memory and change it, and mutable means the address could change?
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _ is a catch all
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Low!"),
            Ordering::Greater => println!("Too High!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }

    }
   
}
