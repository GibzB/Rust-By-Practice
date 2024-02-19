// Guessing game

// A program that generates a random number between 1 and 100 and asks the user to guess the number. 
// The program also tells the user whether their guess is too high or too low until they guess the correct number.
// The program also keeps track of the number of guesses the user makes.

// Import the rand crate
extern crate rand;

// Import the io library from the standard library
use std::io;

// Import the traits from the rand crate
use rand::Rng;

// Import the traits from the io library
use std::cmp::Ordering;


fn main() {
    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Print the secret number
    println!("The secret number is: {}", secret_number);

    // Print a message to the user
    println!("Guess the number!");

    // Create a new, mutable variable called guess
    let mut guess = String::new();

    // Read a line of input from the user and store it in the guess variable
    io::stdin().read_line(&mut guess)
        // Handle the error
        .expect("Failed to read line");

    // Print the user's guess
    println!("You guessed: {}", guess);
}

// Path: Rust-By-Practice/Guessing-Game/src/main.rs
// Compare this snippet from Rust-By-Practice/hello-world/main.rs:
