use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number! Type quit to exit");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); 

        if guess.trim() == "quit" {
            println!("Exiting the program...");
            return;
        }
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
         };

        println!("You guessed: {guess}");

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
