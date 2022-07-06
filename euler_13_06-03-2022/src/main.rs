use std::io::BufRead;
use rug::{Assign, Integer};

fn main() {
    let lines = read_file();
    let mut big_sum = Integer::new();
    for l in lines {
        println!("line: {}\n", l);
        let mut big_num = Integer::new();
        big_num.assign(Integer::parse(l).unwrap());
        println!("big_num: {}\n", big_num);
        big_sum += big_num;
    }
    println!("{}", big_sum);
}

/*
fn generate_int_vec(s: &String) -> Vec<u32> {
    let mut r_vec: Vec<u32> = Vec::new();
    for c in s.chars() {
        let n = match c.to_digit(10) {
            Some(number) => number,
            None => 0,
        };
        r_vec.push(n);
    }
    r_vec
}
*/
fn read_file() -> Vec<String> {
    let file = std::fs::File::open("nums.yui").expect("no such file");
    let buf = std::io::BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}
