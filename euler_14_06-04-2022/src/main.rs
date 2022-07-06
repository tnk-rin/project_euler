fn main() {
    let mut max_length: u64 = 0;
    let mut max_num: u64 = 0;
    for i in 500000..1000000 {
        let curr_length: u64 = collatz_sq_lenth(i);
        if curr_length > max_length {
            max_length = curr_length;
            max_num = i;
        }
    }
    println!("{} : {}", max_num, max_length);
}

fn collatz_sq_lenth(mut n: u64) -> u64 {
    let mut counter: u64 = 1;
    while n != 1 {
        if (n % 2) == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        counter += 1;
    }
    counter
}
