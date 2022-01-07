pub fn run() {
    let x = 10; // int
    // compiler usually identifies the data type
    let y = 2.5; // float
    let z = "c"; // char
    println!("x =  {}, y = {}, z = {}", x, y, z);

    // explicit assigning
    let a:i32 = 10; // int
    let b:f64 = 2.5; // float
    println!("a = {}, b = {}", a, b);

    // boolean data type
    let c = true; // or let c:bool=true||false;

    // assigning boolean values from expression
    let d = 10 < 5;
    println!("c is {} and d is {}", c, d);

    // emoji is char type
    let e = '\u{1F643}';
    println!("{}", e);
}
