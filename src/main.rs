fn main() {
    let number = 9;

    if number < 5 {
        println!("True\n");
    }
    else{
        println!("False\n");
    }

    else_if();
    if_in_let();
    // infinite_loop();
    use_continue();
    println!("\n");
    value_from_loop();

}

// Else if

fn else_if(){
    let number = 10;

    if number %2 ==0{
        println!("Even Number: {}\n",number);
    }

    else if number % 3 ==0 {
        println!("Odd Number: {number}");
    }

    else {
        println!("The number is neither even nor odd.");
    }
}

// if in let

fn if_in_let(){
    let condition = true;
    let number = if condition { 5 } else {6}; // Types of if and else must be same always,or it will give error

    println!("Number : {number}");
}

fn infinite_loop(){
    loop {
        println!("Infinite Loop!"); //CTRL+C stops the code in terminal
    }
}

fn use_continue(){
    for num in 1..=10 {
    if num % 2 == 0 {
        continue; // Skip even numbers
    }

    println!("Number: {}", num);
}

}

fn value_from_loop(){
    let mut counter=0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}
