pub fn run() {

    // declaring and printing array
    let hello = "Hello"; //premitive str
    println!("{}", hello);

    // String
    let hello = String::from("Hemlo");
    println!("{}", hello);

    // push char
    let mut hello = String::from("Hello ");
    println!("{}", hello);
    hello.push('W');
    println!("{}", hello);

    // push string
    hello.push_str("orld!");
    println!("{}", hello);

    // length method
    println!("Length of string is {}", hello.len());

    // capacity in bytes
    println!("Capacity of the string is {} bytes", hello.capacity());

    // check if string is empty
    println!("is empty {}", hello.is_empty());

    // check if string contains a substring
    println!("contains {}", hello.contains("World"));

    // replace in the string
    println!("{}", hello.replace("World", "There"));

    // split bt whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create custom string with capacity
    let mut hello = String::with_capacity(10);
    hello.push('a');
    hello.push('b');
    println!("{}", hello);
}
