use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line...");

    let mut input: i32 = input.trim().parse().expect("you have to enter a number, bro");

    // loop {
    //     if input !=0 {
    //         println!("{}!", input);
    //         input -= 1;
    //     } else {
    //         println!("Liftoff!!");
    //         break;
    //     }
    // };

    while input !=0 {
        println!("{}!", input);
        input -= 1;
    }

    println!("Liftoff!!")
}

