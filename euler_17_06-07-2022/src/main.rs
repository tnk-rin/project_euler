use std::iter::successors;

const ONES: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&str; 10] = [
    "zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty",
    "seventy", "eighty", "ninety",
];

const ORDERS: [&str; 2] = [
    "zero",
    "thousand"
];



fn main() {
    let mut sum: u64 = 0;

    for i in 1..1001 {
        sum += letters_in_num(i);
    }

    println!("{}", sum);
}

fn encode(num: u64) -> String {
    match num {
        0..=19 => ONES[num as usize].to_string(),
        20..=99 => {
            let upper = (num / 10) as usize;
            match num % 10 {
                0 => TENS[upper].to_string(),
                lower => format!("{}{}", TENS[upper], encode(lower)),
            }
        }
        100..=999 => format_num(num, 100, "hundred"),
        _ => {
            let (div, order) =
                successors(Some(1u64), |v| v.checked_mul(1000))
                    .zip(ORDERS.iter())
                    .find(|&(e, _)| e > num / 1000)
                    .unwrap();

            format_num(num, div, order)
        }
    }
}

fn format_num(num: u64, div: u64, order: &str) -> String {
    match (num / div, num % div) {
        (upper, 0) => format!("{}and{}", encode(upper), order),
        (upper, lower) => {
            format!("{}{}and{}", encode(upper), order, encode(lower))
        }
    }
}

fn letters_in_num(n: u64) -> u64 {
    let s: String = encode(n);
    println!("{}",s);
    s.chars().count() as u64
}
