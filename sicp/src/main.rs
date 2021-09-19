fn main() {
    println!("{}", ackerman(1, 10));
    println!("{}", ackerman(2, 4));
    println!("{}", ackerman(3, 3));
}

fn ackerman(x: i32, y: i32) -> i32 {
    if y == 0 {
        0
    } else if x == 0  {
        2 * y
    } else if y == 1 {
        2
    } else {
        ackerman(x-1, ackerman(x, y-1))
    }

}

