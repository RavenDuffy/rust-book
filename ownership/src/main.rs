fn main() {
    numbers();

    string_clone();
}

fn numbers() {
    let x = 5;
    let y = x;

    println!("{x} {y}");

    let x = 6;

    println!("{x} {y}")
}

fn string_clone() {
    let mut s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{s1} {s2}");

    s1 = String::from("hi");

    println!("{s1} {s2}");
}

fn strings() {
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{s1} {s2}");

    // let s1 = String::from("hi");

    // println!("{s1} {s2}");
}
