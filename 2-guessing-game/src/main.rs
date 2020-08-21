use std::io;
use std::str::FromStr;
use std::cmp::Ordering;
use rand::Rng;
use std::result::Result;

fn read_guess() -> Result<u32, <u32 as FromStr>::Err> {
    println!("Input your guess:");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    guess.trim().parse()
}

fn main() {
    let secret = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number!");

    loop {
        let guess = match read_guess() {
            Ok(n) => n,
            Err(_) => {
                println!("That doesn't look like a number");
                continue;
            },
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("You got it!");
                break;
            },
        }
    }
}
