//standard input output library
use std::io;

//Prelude is a list of things that Rust automatically imports in each program
//its kept as small as possible and is focussed on traits which are used
//in almost all Rust programs

//There is "the prelude" and "other preludes"
//std::io falls in the category of other preludes and hence needs to be imported.

fn main() {

    //welcome message to the end user
    println!("Welcome to my guess game");

    //user will enter a number
    println!("Please input your guess");

    //let guess = 5; -> In rust variables are immutable by default
    //meaning once we give the variable a value, the value will not change
    //to make a variable mutable we add mut before the variable name
    //let mut gyess = 5;

    let mut guess = String::new();
    //String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.
    //Each character in the string takes between 1 to 4 bytes of memory

    //like variables, references are also immutable by default.
    //hence we need to add &mut and not just &

    //read_line returns a Result which is an enumeration. 
    //enums are types that can be in one of multiple possible states.
    //each possible state is a variant. 
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        

    println!("You guessed: {guess}");

}
