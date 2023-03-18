//standard input output library
use std::io;

fn main() {

    //welcome message to the end user
    println!("Welcome to my guess game");

    //user will enter a number
    println!("Please input your guess");

    //declare a new string which is mutable
    let mut guess = String::new();
    
    
    //read_line returns a Result which is an enumeration. 
    //enums are types that can be in one of multiple possible states.
    //each possible state is a variant. 
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        

    println!("You guessed: {guess}");

}
