extern crate core;

use std::io;
use rand::Rng;

fn main() {
    print_guess_number();
    ownership_test_1();
    ownership_test_2();
    pointer_test_1();
    mutable_reference_1();
    use_slice_1();
}

fn mutable_reference_1() {
    println!("\nmutable_reference_1: ");
    let mut s = String::from("Hell");
    let r = &mut s;
    r.push_str("o, world");
    println!("r: {}", r);
    println!("r: {}", &r);
}

fn pointer_test_1() {
    println!("\npointer_test_1: ");
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn ownership_test_2() {
    println!("\nownership_test_2: ");
    let s1 = String::from("Hello2");
    let (s2, len) = get_length(s1);
    println!("s2 : {} len: {}", s2, len);
}

fn get_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn ownership_test_1() {
    println!("\nownership_test_1: ");
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
    println!("\nprint_guess_number: ");

    let secret = rand::thread_rng().gen_range(1..101);
    println!("Secret Number: {}", secret);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed number: {}", guess);
}

fn use_slice_1() {
    println!("\nuse_slice_1: ");

    let s = String::from("Hello world");
    let bytes = s.as_bytes();
    let mut f = "";
    for (i, &it) in bytes.iter().enumerate() {
        if it == b' ' {
            f = &s[..i];
            println!("first word: {}", f);
            break;
        }
    }
    if f == "" {
        f = &s[..];
        println!("first word: {}", f);
    }
}