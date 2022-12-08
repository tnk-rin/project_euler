use rug::{Assign, Integer};

fn main() {
    let mut n = Integer::new();
    let mut a = Integer::new();
    n.assign(100);
    for i in 1..100 {
        a.assign(i);
        let na = &n * &a;
        n = Integer::from(na);
        println!("{}", i);
    }

    let s = n.to_string_radix(10);
    println!("{}", s.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>());
}
