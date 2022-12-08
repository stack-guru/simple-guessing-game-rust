use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
