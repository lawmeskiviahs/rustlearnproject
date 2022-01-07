// // traditional struct
// struct Color {
//     red:u8,
//     green:u8,
//     blue:u8
// }
// tuple struct
//struct Color (u8, u8, u8);

// struct incorporating functions
struct Person {
    first_name: String,
    last_name: String
}
impl Person {
    fn new (first: &str, last: &str) -> Person {

        // set name
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }
    fn get_person (&self) -> String {
        // return full name
        format!("{} {}", self.first_name, self.last_name)
    }
}
pub fn run() {

    //export PATH="/home/truler/.local/share/solana/install/active_release/bin:$PATH"

    // initialising structs with functions
    let p = Person::new("Shaivik", "Semwal");
    println!("{} {}", p.first_name, p.last_name);
    println!("{}", p.get_person());
    // initialising traditional struct
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0
    // };
    // c.blue=255;
    // println!("Color: {} {} {}", c.red, c.green, c.blue);

    // initialising a tuple struct
    //let mut c = Color(255, 0, 0);
    //c.2=255;
    //println!("color: {} {} {}", c.0, c.1, c.2);
}
