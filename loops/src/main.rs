fn main() {
    let mut count = 0;

    // loop labelling
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count = {}", count);
    return_value_from_loop();
    countdown();
}

fn return_value_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        
        // to return value from a loop, add the expression after the break statement
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The final value of the loop is {}", result);
}

fn countdown() {
    for num in (1..4).rev() {
        println!("{}",num);
    }
    println!("lift of");
}
