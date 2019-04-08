// To do IO operations, need to bring IO library into scope
// By default, Rust brings only a few types into the scope of every program in the prelude. If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a use statement.
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // will provide a number generator that's `local to the current thread of execution and seeded by the operating system`
    //.gen_range takes two inputs, first one is inclusive, the second is exclusive
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        // use let to create a variable
        // UTF-8 encoded
        // :: indicates associated--static--function/method of String type
        // new creates a new empty string
        // mut before variable name, in this case guess, indicates
        // the variable is mutable--changable
        let mut guess = String::new();


        // could use std::io::io::stdin()...if IO was not imported
        // stdin() returns an instance of std::io::Stdin--type that represents a handleto standard input on terminals
        // read)line method gets input from the users
        // read_line takes whatever user inputs, and returns it as a string
        // & indicates reference, allows multiple parts of code to access one piece of data without duplicating data in memory => smaller foot print
        // like variables, references are immutable by default
        // &mut makes the reference mutable

        io::stdin().read_line(&mut guess)
        // new line because its easier to read the code
        // readline also returns a value-- io::Result
        // There are lots of types named Result in rusts standard library
        // a generic, and specific variants for submodules -- like io::Result
        // Result types are enums
        // For result its either Ok, or Err
        // ok indicates operation succeded, and the value inside ok is the result
        // error indicates operation failed, and the error parameter holds the error--how and why the op. failed.
        // Result purpose to encode error handling
            .expect("Failed to read line");
        // shadow's previous variable instead of erroring out--FEATURE OF RUST
        // guess is reference to previous value
        // trim removes whitespaces including \n
        // parse converts string to u32 type--parser takes the type from the variable
        // : tells rust the Programmer will annotate the type
        // parse returns Result type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // {} placeholder for the value in guess
        // can add infinite {}
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
