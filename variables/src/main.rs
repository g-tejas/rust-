fn main() {
    // in large data structures, using mutable values might be better in terms of performance
    // but for smaller data structures, using immutable might be better for clarity as a trade of
    // off performance
    
    let mut x = 5; // variables are immutable by default
    println!("X is {}", x);
    x = 6;
    println!("X is {}", x);

    // Constant
    // The convention for naming constants in Rust is upper case w/ underscore between words
    // Constants are immutable forever unlike variables
    // You can only set constants to constant expressions, not values that can only be computed at
    // runtime, because the compiler will check this at compile time not run time.
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("The constant is {}", THREE_HOURS_IN_SECONDS);

    // Shadowing
    // You can use the same variable again (even immutable) by writing let infront of it, and the
    // second variable is what the compiler sees when it uses the variable
    let shadowed_variable = 5;
    
    // We have to use "let" when shadowing
    // shadowed_variable = shadowed_variable + 1 > this will return an error
    let shadowed_variable = shadowed_variable + 1;

    {
        let shadowed_variable = shadowed_variable * 2;
        println!("The value of shadowed variable in the inner scope is {}", shadowed_variable);
    }

    println!("The value of the shadowed variable is {}", shadowed_variable);
    
    // Shadowed variables allow us to use different types of variables with the same name
    // where it would be messy. For e.g, spaces, spaces_str, spaces_int, will be messy
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);
    // then why not use mutable values? 
    // You can't change the type with mutable values
    let mut chicken = "    ";
    chicken = chicken.len(); // This will return a type error

}


