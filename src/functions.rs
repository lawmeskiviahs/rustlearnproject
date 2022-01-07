pub fn run() {

    // funtion calling
    hello("Morning", "Shaivik");

    // binding returned value to a variable
    let x = add(3, 5);
    println!("add: {}", x);

    // Closure
    let sum = |n1: i32, n2: i32| n1 + n2;
    println!("closure sum: {}", sum(8, 12));
}

// function with input parameters
fn hello(greet : &str, name : &str) {
    println!("Good {} {}.", greet, name);
}

//function returning a value
fn add(n1: i32, n2: i32) -> i32 {

    // expression not a statement
    n1 + n2
}
