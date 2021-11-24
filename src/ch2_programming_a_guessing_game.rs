use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn secret_number() {
    println!("Guess number!");

    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("Secret number: {}", secret_number);

    loop {
        println!("Input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Invalid input");
                continue;
            } // _ is a catchall value; we want to match all Err values, no matter what information they have inside them.
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
