use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    match io::stdin().read_line(&mut guess) {
        Ok(_s) => println!("You guessed: {}", guess),
        Err(_error) => println!("Failed to read line"),
    }
}
