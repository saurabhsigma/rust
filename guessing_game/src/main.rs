use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;
fn main() {
    println!("Hello, world!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("the secret number is {}", secret_number);

    loop {
        println!("Please input you guess. ");

        // let guess: String = String ::new()
        let mut guess = String::new(); // to make variable mutable we add mut before the name of the variable

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        // let guess: u32 = guess.trim().parse().expect("PLZ type a number"); //lets now change it using match so that it doesnt panic and keep on the guessing game untill ser wins
        // and for that we will use match
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Plz enter a valid number nothing else!");
                continue;},
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win".green());
                break;
            }
        }
    }
}
