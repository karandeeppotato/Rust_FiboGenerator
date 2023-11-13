// Generate the nth fibonacci number.

use std::io;

fn fibo_generator(n: usize) -> usize {
    if n == 1 {
        return 0
    } else if n == 2 {
        return 1
    }

    let mut i : usize = 2;
    let mut t1: usize = 0;
    let mut t2: usize = 1;
    let mut res: usize = 0;

    while i < n {
        res= t1 + t2;
        t1 = t2;
        t2 = res;

        i += 1;
    }

    res
}

fn main() {
    println!("Fibonacci Number generator");

    println!("Enter the value of n: ");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Cannot read line");
    let n: usize = n.trim().parse().expect("Invalid Entry");

    let result = fibo_generator(n);

    println!("The {n}th number in fibonacci sequence is: {result}")
}