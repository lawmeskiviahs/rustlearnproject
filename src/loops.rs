pub fn run() {

    // for range
    for num in 1..100 {
        if num % 15 == 0 {println!("FizzBuzz {}", num);}
        else if num % 5 == 0 {println!("Buzz {}", num);}
        else if num % 3 == 0 {println!("Fizz {}", num);}
    }

    //while range
    let mut count = 0;
    while count<100 {
        if count % 15 == 0 {println!("FizzBuzz {}", count);}
        else if count % 5 == 0 {println!("Buzz {}", count);}
        else if count % 3 == 0 {println!("Fizz {}", count);}
        count+=1;
    }
}
