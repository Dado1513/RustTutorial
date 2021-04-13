#![allow(unused)]

use std::io;

fn main() {

    // ******************************************************************************************* //
    // Variables and Mutability

    let x = 5; // immutable variable
    println!("The value of x is: {}", x);
    let mut y = 34; // mutable variable
    println!("The value of y is: {}", y);
    y = 29; // mutable variable
    println!("The value of y is: {}", y);

    // Costant Like immutable variables, constants are values that are bound to a name and are
    // not allowed to change, but there are a few differences between constants and variables.
    // First, you aren’t allowed to use mut with constants.
    // Constants aren’t just immutable by default—they’re always immutable.
    // You declare constants using the const keyword instead of the let keyword,
    // and the type of the value must be annotated
    // Constants can be declared in any scope, including the global scope,
    // which makes them useful for values that many parts of code need to know about.
    // The last difference is that constants may be set only to a constant expression,
    // not the result of a function call or any other value that could only be computed at runtime.
    // Here’s an example of a constant declaration where the constant’s name is
    // MAX_POINTS and its value is set to 100,000. (Rust’s naming convention for constants
    // is to use all uppercase with underscores between words, and underscores can be
    // inserted in numeric literals to improve readability):
    const MAX_POINTS: u32 = 100_000;

    // Shadowing
    // you can declare a new variable with the same name as a previous variable,
    // and the new variable shadows the previous variable. Rustaceans
    // say that the first variable is shadowed by the second, which means that the
    // second variable’s value is what appears when the variable is used.
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
    // This program first binds x to a value of 5. Then it shadows x by repeating let x =, taking
    // the original value and adding 1 so the value of x is then 6. The third let
    // statement also shadows x, multiplying the previous value by 2 to give x a final value of 12

    // Shadowing is different from marking a variable as mut, because we’ll get a
    // compile-time error if we accidentally try to reassign to this variable without using
    // the let keyword. By using let, we can perform a few transformations on a value but have
    // the variable be immutable after those transformations have been completed.

    // The other difference between mut and shadowing is that because we’re effectively
    // creating a new variable when we use the let keyword again, we can change the type of
    // the value but reuse the same name.
    let spaces = "   ";
    let spaces = spaces.len();
    // This construct is allowed because the first spaces variable is a string type and the
    // second spaces variable, which is a brand-new variable that happens to have the same name
    // as the first one, is a number type. Shadowing thus spares us from having to
    // come up with different names, such as spaces_str and spaces_num; instead,
    // we can reuse the simpler spaces name.

    // ******************************************************************************************* //
    // Data Types

    // Every value in Rust is of a certain data type, which tells Rust what kind of data is being
    // specified so it knows how to work with that data. We’ll look at two data type subsets: scalar and compound.

    // Keep in mind that Rust is a statically typed language, which means that it must know the types
    // of all variables at compile time.
    let guess: u32 = "42".parse().expect("Not a number!");
    // If we don’t add the type annotation here, Rust will display an error

    // Scalar Types
    // A scalar type represents a single value. Rust has four primary scalar types: integers,
    // floating-point numbers, Booleans, and characters. You may recognize these
    // from other programming languages. Let’s jump into how they work in Rust.

    // Integer Types
    // 8-bit	i8	u8
    // 16-bit	i16	u16
    // 32-bit	i32	u32
    // 64-bit	i64	u64
    // 128-bit	i128	u128
    // arch	isize	usize
    // An integer is a number without a fractional component.
    // This type declaration indicates that the value it’s associated with should be an
    // unsigned integer (signed integer types start with i, instead of u) that takes up 32 bits of space.
    // Additionally, the isize and usize types depend on the kind of computer your program is running on:
    // 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

    // Floating-Point Types
    // Rust also has two primitive types for floating-point numbers, which are numbers with decimal points.
    // Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively.
    // The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is
    // capable of more precision.

    let x = 2.0;
    let y: f32 = 3.0;
    println!("X value is {}, y value is {}", x, y);

    // Numeric Operations
    // Rust supports the basic mathematical operations you’d expect for all of the
    // number types: addition, subtraction, multiplication, division, and remainder.

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let reminder = 43 % 5;

    // Boolean Type
    let t = true;
    let f : bool = false;

    // Character type
    // So far we’ve worked only with numbers, but Rust supports letters too.
    // Rust’s char type is the language’s most primitive alphabetic type, and the following code
    // shows one way to use it. (Note that char literals are specified with single quotes, as opposed
    // to string literals, which use double quotes.)
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    // Rust’s char type is four bytes in size and represents a Unicode Scalar Value,
    // which means it can represent a lot more than just ASCII.

    // Compound Types
    // Compound types can group multiple values into one type. Rust has two primitive compound
    // types: tuples and arrays.
    // A tuple is a general way of grouping together a number of values with a variety of types
    // into one compound type. Tuples have a fixed length: once declared, they cannot grow
    // or shrink in size.
    // We create a tuple by writing a comma-separated list of values inside parentheses.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // The variable tup binds to the entire tuple, because a tuple is considered a single compound element.
    // To get the individual values out of a tuple, we can use pattern matching
    // to destructure a tuple value, like this:
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    // This program first creates a tuple and binds it to the variable tup. It then uses a
    // pattern with let to take tup and turn it into three separate variables, x, y, and z.
    // This is called destructuring, because it breaks the single tuple into three parts.
    // Finally, the program prints the value of y, which is 6.4
    let x_value = tup.0;
    let y_value = tup.1;
    let z_value = tup.2;
    println!("The value of x {}, y {}, z {}", x_value, y_value, z_value);

    // Array type
    // Another way to have a collection of multiple values is with an array.
    // Unlike a tuple, every element of an array must have the same type.
    // Arrays in Rust are different from arrays in some other languages
    // because arrays in Rust have a fixed length, like tuples
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];


    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    // accessing array elements
    let first = a[0];
    let second = a[1];

    // Invalid Array Element Access
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

    // This code compiles successfully. If you run this code using cargo run and enter 0, 1, 2, 3,
    // or 4, the program will print out the corresponding value at that index in the array.
    // If you instead enter a number past the end of the array, such as 10,
    // you'll see output like this:

    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

}
