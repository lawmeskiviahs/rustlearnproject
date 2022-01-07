pub fn run() {
    let mut x = 20;
    if x>=18 {
        println!("18+");
    }
    else {
        println!("Minor");
    }

    // shorthand if
    let leg:bool = if x >=21 {true} else {false};
    println!("Is legal: {}", leg);
}
