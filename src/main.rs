fn main() {
    let number = 9;

    if number < 5 {
        println!("True\n");
    } else {
        println!("False\n");
    }

    else_if();
    if_in_let();
    // infinite_loop();
    use_continue();
    println!("\n");

    value_from_loop();
    println!("\n");

    named_loops();
    println!("\n");

    while_loop();
    println!("\n");

    rev_for_loop();
    println!("\n");

    for_loop();
}

// Else if

fn else_if() {
    let number = 10;

    if number % 2 == 0 {
        println!("Even Number: {}\n", number);
    } else if number % 3 == 0 {
        println!("Odd Number: {number}");
    } else {
        println!("The number is neither even nor odd.");
    }
}

// if in let

fn if_in_let() {
    let condition = true;
    let number = if condition { 5 } else { 6 }; // Types of if and else must be same always,or it will give error

    println!("Number : {number}");
}

fn infinite_loop() {
    loop {
        println!("Infinite Loop!"); //CTRL+C stops the code in terminal
    }
}

fn use_continue() {
    for num in 1..=10 {
        if num % 2 == 0 {
            continue; // Skip even numbers
        }

        println!("Number: {}", num);
    }
}

fn value_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}

// Named Loops

fn named_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
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
    println!("End count = {count}");
}

// While Loop

fn while_loop(){
    let mut number = 3;

    while number !=0 {
        println!("{number}");

        number -=1;
    }

    println!("TakeOFF!!");
}

// For Loops

fn rev_for_loop(){
    for number in (1..=4).rev(){
        println!("{number}");
    }
    println!("LiftOFF!!");
}

//Printing Array

fn for_loop(){
    let a = [1,2,3,4,5];

    for element in a{
        println!("{element}");
    }
}
