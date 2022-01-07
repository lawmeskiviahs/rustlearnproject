pub fn run() {

    // declaring and initializing
    let mut x:[i32;5]=[1, 2, 3, 4, 5];
    println!("{:?}", x);

    // printing single element
    println!("{}", x[1]);

    // reassigning values
    x[1]=10;
    println!("{}", x[1]);

    // length of the array
    println!("Length of array is {}", x.len());

    // slicing in the arrays
    let slice : &[i32] = &x[1..3];
    println!("{:?}", slice);
}
