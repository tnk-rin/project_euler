fn main() {
    let mut curr_triangle = 0i64;
    let mut next_triangle_add = 1i64;
    let mut max_factors = 0u32;

    while max_factors <= 500 {
        (curr_triangle, next_triangle_add) = get_next_triangle(curr_triangle, next_triangle_add);
        max_factors = num_factors(curr_triangle);
    }
    println!("n: {}\n\tnext add: {}\n\tfactors: {}\n", curr_triangle, next_triangle_add, num_factors(curr_triangle));
}

fn num_factors(n: i64) -> u32 {
    let mut f = 0u32;
    for i in 1..((n as f64).sqrt() as i64 + 1) {
        if (n % i) == 0 {
            if (n / i) == i {
                f += 1;
            } else {
                f += 2;
            }
        }
    }

    f
}

fn get_next_triangle(tri: i64, next_add: i64) -> (i64, i64) {
    (tri + next_add, next_add + 1)
}
