// things outside the prelude must be imported using 'use' (the prelude is the set of functions
// available by default)
// import using 'use' and use '::' to get specific things in a namespace
use std::io;
// Ordering is an enum that contains the types: Less, Greater, Equal
use std::cmp::Ordering;

// this imports the Rng trait which brings random number generators into scope
use rand::Rng;

fn main() {
    // rand::rng() is a generator that is local to the current exectution thread
    // random_range() is a function that locks the randomly generated number within the defined
    // range
    // the range inside random_range() takes a range expression (inclusive)
    let secret_num = rand::rng().random_range(1..=100);
    println!("The secret number is: {secret_num}");

    // loop creates an infinite loop
    loop {
        println!("Guess the number: ");

        // let creates a new variable
        // mut sets the variable to be mutable (all variables are immutable by default)
        // String is part of the prelude and is a utf-8 encoded string (that is growable)
        // A string is an object and must be instantiated with ::new()
        let mut guess = String::new();

        // stdin() allows user input handling
        io::stdin()
            // read_line() reads in whatever the user types and APPEND to a string
            // &mut is a reference (denoted by &), which allows access to the guess value without
            // copying data
            // &mut, mut is specified again because references are by default immutable
            .read_line(&mut guess)
            // read_line returns one of two enums: Ok or Err
            // in the Err case, an error has occurred which will trigger the .expect function (if it's
            // defined)
            .expect("Failed to read line");

        // this is called shadowing and it allows variables to be overwritten with new values (this is
        // used specifically to overwrite one type to another)
        // trim() removes any trailing and leading spaces
        // parse() tries to convert a string to another type (that type is specified in the variable
        // type, in this case u32)
        // u32 is an unsigned 32bit int
        // expect() catches any errors with the previous statement (errors are the type Err)
        // let guess: u32 = guess.trim().parse().expect("Please enter a number!");

        // this version of guess only takes a value that can be successfully parsed as a number, if
        // not continue is run to skip the current iteration of the loop
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        // println! allows variables to be used with {[variable name]}
        // if {} are used (and remain empty) it will try to fill with any remaining arguments
        println!("You guessed: {}", guess);

        // a match expression is made of 'arms', where each arm is a matching pattern
        // each arm is checked (sequentially)
        // cmp() is a function that compares two values (the initial variable and a reference to
        // the other variable)
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
