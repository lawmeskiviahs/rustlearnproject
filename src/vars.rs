pub fn run() {

    // declaring variables
    let name = "Shaivik";
    let age = 20;
    println!("My name is {} and I'm {}", name, age);

    // mutability
    let name="Shaivik";
    let mut age=20;
    println!("My name is {} and I'm {}", name, age);
    age=21;
    println!("My name is {} and I'm {}", name, age);

    // assigning multiple variables
    let (name, age) = ("Shaivik", 20);
    println!("My name is {} and I'm {}", name, age);
}
