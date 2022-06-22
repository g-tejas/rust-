fn main() {
    another_function(5);
    
    // expressions do not include an ending ; semicolon, if you do, it will turn into a statement
    //let x = 6; // is a statement
    
    let x = five();
    println!("the value of x is {}", x);

    let y = plus_one(x);
    println!("The value of x plus one is {}", y);
}


// in arguments, you MUST provide type or else the rust compiler will be angry
fn another_function(x: i32) {
    println!("The value of x is {}", x);
}

// functions that return values
// rust functions return the last value that comes from the last expression but you can return
// early by using 'return'.
// you don't have to name the return value but you have to declare it's type.
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // x + 1; will return a error because a statement doesn't return a value, only expressions do.
    // plus_one fn says that it returns an i32 integer but statements don't return value, so we get
    // a mismatch error.
}
