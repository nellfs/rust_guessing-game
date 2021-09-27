//Source: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#allowing-multiple-guesses-with-looping

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number= rand::thread_rng().gen_range(1..101);

    loop {
        
    println!("Guess the number!");
    println!("Please input your guess.");

    //println!("The secret number is: {}", secret_number);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue
    };


    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
            }
        }
    }
}