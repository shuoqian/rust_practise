use std::io;

fn main() {
    println!("Welcome to the guess game");
    println!("Please input your guess number (integer):");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");
    println!("Your guess is: {}", guess);
}
