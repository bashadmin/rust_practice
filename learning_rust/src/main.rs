fn main() {
    // Here is a comment
    // This is a function that sums two numbers
    // The function returns the sum of the two numbers
    // The function takes two parameters
    /*This is
    a 
    multiline comment */

    /* meta programming is code that writes other codes, and you do this by using macros. */

    print!("Hello,
     world 
     from\n 
     rust!
     ");
    println!("println uses a new line");

    println!("This part of the text will be overwritten\r value is: ${}{}.", 10,20);

    // If you're trying to print a directory path, you can use the r prefix to make it raw? Also, you can use the # prefix to print the debug representation of a value? The \ can be used to make directory paths?

    println!("r prefix: r#\"C:\\Users\\Documents\\\"#");

    println!("The value of 10 + 20 is: {}", 10 + 20);

    println!("The value of 10 - 20 is: {}", 10 - 20);

    println!("The value of 10 * 20 is: {}", 10 * 20);

    println!("The value of 10 / 20 is: {}", 10 / 20);

    println!("The value of 10 % 20 is: {}", 10 % 20);

    println!("The value of 10.0 + 20.0 is: {}", 10.0 + 20.0);

    println!("The value of 10.0 - 20.0 is: {}", 10.0 - 20.0);

    println!("The {name} is done by passing {values} into the {function} function.", name="function", values="values", function="function");

    println!("The {0} is done by passing {1} into the {2} function.", "function", "values", "function");

    println!("\n Doing {2} for {1} years and I'm {0} it.", "loving", 0, "rust");

    println!("{}", 10 + 20);

    println!("{}", 10 - 20);

    println!("{}", 10 * 20);

    println!("{}", 10 / 20);
}