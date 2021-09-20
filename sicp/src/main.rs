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
    pascal_recursive(10);
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

fn pascal_recursive(row_index: u64) {
    let mut counter = 0;
    println!("{:?}", vec![1]);
    while counter <= row_index {
        counter += 1;
        println!("{:?}", find_pascal_line(counter))
    }

    fn find_pascal_line(row_index: u64) -> Vec<u64> {
        fn next_line(line: Vec<u64>) -> Vec<u64> {
            let mut next_line: Vec<u64> = vec![0];
            let mut counter:u64 = 0;
            loop {
                counter +=1;
                next_line.push(line[counter as usize] + line[(counter-1) as usize]);
                if counter == line.len() as u64 - 1{
                    next_line.push(0);
                    break next_line
                }
            }
        }

        let mut counter:u64 = 0;
        let mut current_line = vec![0,1,0];
        loop {
            counter += 1;
            if counter == row_index {
                let mut pascal_line = next_line(current_line);
                pascal_line.remove(0);
                pascal_line.remove(pascal_line.len() - 1);
                break pascal_line
            }
            current_line = next_line(current_line)
        }
    }
}
