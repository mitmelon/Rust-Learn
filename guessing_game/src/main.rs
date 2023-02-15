use rand::Rng;
use std::cmp::Ordering;
use std::io;

//version 1.0.0
//Future updates - 
//   Only allow 3 times play for 24 hours
//   Create a token wallet for user and give them token if they win (Token will be redeemed only at the Manowish platform.)

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        //Receive user input from command line
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //Convert guess to int
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
