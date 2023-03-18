//standard input output library
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    //generate a random number which will be a secret
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //print random number generated
    //println!("The random number generated is {secret_number}");

    //welcome message to the end user
    println!("Welcome to my guess game");

    //game runs until a user guesses the correct number or presses ctrl + c
    loop {
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
            

        //use Shadowing to convert guess to u32
        //if there is an error while converting to u32 (user entered a value that is not a number)
        //then ignore this input and continue the loop execution
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, //_ is catchall errors
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            },    
        }
    }

}
