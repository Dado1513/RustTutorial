fn main() {
    another_function();
    another_function_with_parameter(34);
    another_function_with_double_parameter(34, 45);

    // Function bodies contain statements and expressions
    // Function bodies are made up of a series of statements optionally ending in an expression.
    // So far, we’ve only covered functions without an ending expression,
    // but you have seen an expression as part of a statement.
    let x = 5;

    let y = {
        let x = 3;
        x + 1 // voluta la mancanza del punto e virgola
    };
    // is a block that, in this case, evaluates to 4. That value gets bound to y as part of the
    // let statement. Note the x + 1 line without a semicolon at the end, which is unlike most
    // of the lines you’ve seen so far. Expressions do not include ending semicolons.
    // If you add a semicolon to the end of an expression, you turn it into a statement,
    // which will then not return a value.
    // Keep this in mind as you explore function return values and expressions next.
    println!("The value of x is {}", x);
    println!("The value of y is: {}", y);

    let value = function_with_return_values();
    println!("Value is {}", value);
    println!("Value+1 is {}", plus_one(value));


}

fn another_function () {
    println!("Another function.");
}

fn another_function_with_parameter(x: i32) {
    println!("The value of x is {}", x)
}

fn another_function_with_double_parameter(x: i32, y:i32) {
    println!("The value of x is {}, y is {}", x, y)
}

fn function_with_return_values() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}