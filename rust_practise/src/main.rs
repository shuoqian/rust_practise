extern crate core;

use std::io;
use rand::Rng;

fn main() {
    print_guess_number();
    ownership_test_1();
    ownership_test_2();
}

fn ownership_test_2() {
    let s1 = String::from("Hello2");
    let (s2, len) = get_length(s1);
    println!("s2 : {} len: {}", s2, len);
}

fn get_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn ownership_test_1() {
    // after assign the reference, the previous one is invalid to use
    let s1 = String::from("Hello");
    println!("s1: {}", s1);
    let s2 = s1;
    println!("s2: {}", s2);
    //println!("s1: {}", s1);

    // close for deep copy
    let s3 = s2.clone();
    println!("s3: {}", s3);

    // integer works, primitive type works
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    // string works, primitive type works
    let x = "hello";
    let y = x;
    println!("x: {}, y: {}", x, y);
}

fn print_guess_number() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1..101);
    println!("Secret Number: {}", secret);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed number: {}", guess);
}