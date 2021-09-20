use std::time::Instant;
use std::vec::Vec;

fn main() {
    println!("{}", ackerman(1, 10));
    println!("{}", ackerman(2, 4));
    println!("{}", ackerman(3, 3));
    println!("########### Exercise 1.11");
    println!("{}", f_iter(30));
    println!("{}", f_recurse(30));
    let mut before = Instant::now();
    f_iter(30);
    println!("Iterative took {:.2?}", before.elapsed());
    before = Instant::now();
    f_recurse(30);
    println!("Recurse took {:.2?}", before.elapsed());
    println!("########### Exercise pascal");
    println!("{:?}", next_line(vec![0, 1, 1, 0]));
    println!("{:?}", next_line(next_line(vec![0, 1, 1, 0])));
}

fn ackerman(x: i32, y: i32) -> i32 {
    if y == 0 {
        0
    } else if x == 0 {
        2 * y
    } else if y == 1 {
        2
    } else {
        ackerman(x - 1, ackerman(x, y - 1))
    }
}

// Exercise 1.11

fn f_iter(n: u64) -> u64 {
    fn f_step(n: u64, f_n_minus_3: u64, f_n_minus_2: u64, f_n_minus_1: u64, count: u64) -> u64 {
        let mut new_result: u64 = f_n_minus_1 + (2 * f_n_minus_2) + (3 * f_n_minus_3);

        if count == n {
            new_result
        } else if count < 3 {
            new_result = count;
            f_step(n, f_n_minus_2, f_n_minus_1, new_result, count + 1)
        } else {
            f_step(n, f_n_minus_2, f_n_minus_1, new_result, count + 1)
        }
    }

    f_step(n, 0, 1, 2, 0)
}

fn f_recurse(n: u64) -> u64 {
    if n < 3 {
        n
    } else {
        f_recurse(n - 1) + (2 * f_recurse(n - 2)) + (3 * f_recurse(n - 3))
    }
}

fn next_line(line: Vec<i32>) -> Vec<i32> {
    let mut next_line: Vec<i32> = vec![0];
    let mut counter:i32 = 0;
    loop {
        counter +=1;
        next_line.push(line[counter as usize] + line[(counter-1) as usize]);
        if counter == line.len() as i32 - 1{
            next_line.push(0);
            break next_line
        }
    }
}

