use std::io;

fn main() {
    let a = [1, 3, 4, 5, 7];

    println!("Please enter your desired array index:");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("did you enter a number?");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    )
}
