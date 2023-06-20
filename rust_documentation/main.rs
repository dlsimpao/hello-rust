use std::io;
use std::cmp::Ordering;
use rand::Rng; //rand = 0.8.5

fn main() {
    println!("Guess what number I'm thinking.");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Come on. Take a guess!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Little on the low side"),
            Ordering::Greater => println!("Little on the high side"),
            Ordering::Equal => {
                println!("It was just right!");
                break;
            }
        }
    }
}