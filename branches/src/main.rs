fn main() {
    println!("Control Flow");

    let x = 5;

    if x < 5 {
        println!("Number is lesser than 5");
    // rust will only run the first code block that returns true, so even if the later code blocks
    // are true, it won't run.
    } else if x > 5 {
        println!("Number is equal to or greater than 5");
    }
    
    let y = 3;
    
    // unlike python where this will return true, if statements in Rust need explicit boolean
    // values
    if y {
        println!("y is {}", y);
    }
    let condition = true;
    let number = if condition {5} else {6};
    // rust needs to know at compile time, what the type of the returned variables are
    // let number = if condition {5} else {"six"}; will return a mismatch error
}

