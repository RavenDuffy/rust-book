fn main() {
    let fib_num = generate_nth(15);
    println!("{fib_num}");
}

fn generate_nth(n: u16) -> u64 {
    let sqrt_5 = 5f64.sqrt(); // doesn't need a type because it's implied
    let n_f64 = f64::from(n); // same as above
    let f_sum: f64 = (1.0 + sqrt_5).powf(n_f64) / (2f64.powf(n_f64) * sqrt_5);
    println!("{f_sum}");
    (f_sum + 0.5) as u64 // round then return
}
