fn main(){
    // An if expression allows you to branch your code depending on conditions.
    // You provide a condition and then state,
    // “If this condition is met, run this block of code. If the condition is not met, do not run this block of code.”
    let number = 3;
    // All if expressions start with the keyword if, which is followed by a condition.
    // In this case, the condition checks whether or not the variable number has a value less than 5.
    // The block of code we want to execute if the condition is true is placed immediately
    // after the condition inside curly brackets.
    // Blocks of code associated with the conditions in if expressions are sometimes called arms,
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // repetition with loops
    // The loop keyword tells Rust to execute a block of code over
    // and over again forever or until you explicitly tell it to stop.
    // loop {
    //         println!("again!");
    //     }


    // Returning value from Loops
    // One of the uses of a loop is to retry an operation you know might fail, such as checking whether
    // a thread has completed its job. However, you might need to pass the result of that
    // operation to the rest of your code. To do this, you can add the value you want returned
    // after the break expression you use to stop the loop; that value will be returned out of
    // the loop so you can use it, as shown here:
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // Conditional Loops with While
    // It’s often useful for a program to evaluate a condition within a loop. While the condition
    // is true, the loop runs. When the condition ceases to be true, the program calls break,
    // stopping the loop. This loop type could be implemented using a combination of loop,
    // if, else, and break; you could try that now in a program, if you’d like.

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");


    // Looping through a collection with for
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // The safety and conciseness of for loops make them the most commonly used loop construct in Rust.
    // he way to do that would be to use a Range, which is a type provided by the standard library that
    // generates all numbers in sequence starting from one number and ending before another number.
    //
    // Here’s what the countdown would look like using a for loop and another method we’ve
    // not yet talked about, rev, to reverse the range:

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}