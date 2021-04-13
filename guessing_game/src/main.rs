use std::io;
use rand::Rng; // --> the RNG trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");


    // rand:thread_rng --> we’re adding two lines in the middle. The rand::thread_rng function will
    // give us the particular random number generator that we’re going to use: one that is local
    // to the current thread of execution and seeded by the operating system
    // Then we call the gen_range method on the random number generator.
    // This method is defined by the Rng trait that we brought into scope with the use rand::Rng statement.
    // The gen_range method takes two numbers as arguments and generates a random number between them.
    // It’s inclusive on the lower bound but exclusive on the upper bound,
    // so we need to specify 1 and 101 to request a number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);
    println!("Please input your guess:");


    let mut guess = String::new(); // mutable string
    // :: syntax in the ::new line indicates that new is an associated function of the String type --> static method
    //let foo:u32 = 5; // immutable value u32

    // If we hadn’t put the use std::io line at the beginning of the program,
    // we could have written this function call as std::io::stdin.

    // the stdin function --> return an instance of std:io::Stdin --> type to handel the standard input from terminal

    // read_line(&mut guess) --> calls read_line method on the standard input handle to get input from the user.

    // The job of read_line is to take whatever the user types into standard input and place that into a string,
    // so it takes that string as an argument.
    // The string argument needs to be mutable so the method can change the string’s content by adding the user input.

    // The & indicates that this argument is a reference, which gives you a way to let multiple
    // parts of your code access one piece of data without needing to copy that data into memory multiple times.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // read_line puts what the user types into the string we’re passing it,
    // but it also returns a value—in this case, an io::Result
    // Rust has a number of types named Result in its standard library:
    // a generic Result as well as specific versions for submodules, such as io::Result.

    // The Result types are enumerations, often referred to as enums.
    // An enumeration is a type that can have a fixed set of values, and those values are called
    // the enum’s variants

    // For Result, the variants are Ok or Err. The Ok variant indicates the operation was successful,
    // and inside Ok is the successfully generated value.

    // The Err variant means the operation failed, and Err contains information about how or why the
    // operation failed.

    // The purpose of these Result types is to encode error-handling information

    //  An instance of io::Result has an expect method that you can call.
    // If this instance of io::Result is an Err value, expect will cause the program to crash and
    // display the message that you passed as an argument to expect

    // f the read_line method returns an Err, it would likely be the result of an error coming from
    // the underlying operating system. If this instance of io::Result is an Ok value,
    // expect will take the return value that Ok is holding and return just that value to you so
    // you can use it. In this case, that value is the number of bytes in what the user
    // entered into standard input.

    println!("You guessed: {}", guess);
    //println!("{}", foo);

    // Comparing the Guess to the Secret Number
    // Ordering is another enum, but the variants for Ordering are Less, Greater, and Equal.
    // A match expression is made up of arms. An arm consists of a pattern and the code that should
    // be run if the value given to the beginning of the match expression fits that arm’s pattern.
    // Rust takes the value given to match and looks through each arm’s pattern in turn. --> switch

    // first error cmp(&secret_number)  --> Rust has a strong, static type system. However, it also has type inference
    // When we wrote let mut guess = String::new(), Rust was able to infer that guess should be a
    // String and didn’t make us write the type.
    // The secret_number, on the other hand, is a number type. A few number types can have a value between 1 and 100
    // i32, a 32-bit number; u32, an unsigned 32-bit number; i64, a 64-bit number; as well as others.
    // Ultimately, we want to convert the String the program reads as input into a real number type so we
    // can compare it numerically to the secret number. We can do that by adding another line to the main function body

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    // We create a variable named guess. But wait, doesn’t the program already have a variable named guess?
    // It does, but Rust allows us to shadow the previous value of guess with a new one.

    // The trim method on a String instance will eliminate any whitespace at the beginning and end.
    // The parse method on strings parses a string into some kind of number. Because this method can parse
    // a variety of number types, we need to tell Rust the exact number type we want by using let guess: u32.
    // The colon (:) after guess tells Rust we’ll annotate the variable’s type.
    // Rust has a few built-in number types; the u32 seen here is an unsigned, 32-bit integer.
    // It’s a good default choice for a small positive number.
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),

    }

    println!("Start Infinite Loop");
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // if i want handling invalid input
        // Switching from an expect call to a match expression is how you generally
        // move from crashing on an error to handling the error.
        // Remember that parse returns a Result type and Result is an enum that has the variants Ok or Err.
        // We’re using a match expression here, as we did with the Ordering result of the cmp method

        // If parse is able to successfully turn the string into a number, it will return an Ok value
        // that contains the resulting number. That Ok value will match the first arm’s pattern,
        // and the match expression will just return the num value that parse produced and put inside
        // the Ok value. That number will end up right where we want it in the new guess variable we’re creating.
        // If parse is not able to turn the string into a number, it will return an Err value that
        // contains more information about the error. The Err value does not match the Ok(num) pattern
        // in the first match arm, but it does match the Err(_) pattern in the second arm.
        // The underscore, _, is a catchall value; in this example, we’re saying we want to match all
        // Err values, no matter what information they have inside them. So the program will execute
        // the second arm’s code, continue, which tells the program to go to the next iteration of
        // the loop and ask for another guess. So, effectively, the program ignores all errors
        // that parse might encounter!
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please insert a number or positive number");
                continue
            }
        };
        println!("You guessed: {}", guess);
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