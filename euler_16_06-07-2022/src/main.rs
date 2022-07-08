use rug::{Assign, Integer};

fn main() {
    let mut big_num = Integer::new();
    let mut two = Integer::new();
    let mut big_sum = Integer::new();

    two.assign(2);
    big_num.assign(2);
    for _i in 1..1000{
        let ab_ref = &big_num * &two;
        big_num = Integer::from(ab_ref);
    }

    for n in generate_int_vec(&big_num) {
        let n_int = Integer::from(n);
        let ab_ref = &big_sum + &n_int;
        big_sum = Integer::from(ab_ref);
    }


    println!("{}", big_sum);
}

fn generate_int_vec(n: &Integer) -> Vec<u32> {
    let mut r_vec: Vec<u32> = Vec::new();
    let s = n.to_string();
    for c in s.chars() {
        let n = match c.to_digit(10) {
            Some(number) => number,
            None => 0,
        };
        r_vec.push(n);
    }
    r_vec
}
