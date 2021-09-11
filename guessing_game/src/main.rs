//importing the i/o library which is a part of the standard library
use std::io;
use rand::Rng; //Rng is a trait, it defines methods that random number generators implement
use std::cmp::Ordering; 

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        //variables are immutable by default
        //:: means that new is an associated function of String, 
        //associated functions operate on a type and not instance, also known as static method
        let mut guess = String::new();

        //rand:thread_rng() produces a random number generator that is local to the current thread
        //of execution and seeded by the OS
        //gen_range() is defined by the Rng trait, generates a random number within a range, inclusive lower,
        //exclusive upper

        //io has an associated function called stdin, that reads the input of the user
        io::stdin()
            .read_line(&mut guess) //& argument is as a reference, also immutable by default, prevents having to re-write values to memory
            .expect("Failed to read line");

        //variable shadowing, used when wanting to change the type of a variable
        //reuse the variable name guess, but as an i32
        //trim() eliminates whitespace and special characters like \n
        //parse() converts the string into a type of number, u32 for us
        //: u32 annotes the new type for the variable guess
        //parse returns a Result type as it may return an error, an expect is used to deal with it
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        //cmp returns a variant of the ordering enum
        //match is used to decide which output should be selected based on the ordering returned
        //secret_number is inferred to be u32 as guess is u32
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
