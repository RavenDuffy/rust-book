fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{five:?} {six:?} {none:?}");

    let t = Some(3u8);
    if let Some(max) = t {
        println!("The max is now {max}");
    }
    println!("{t:?}");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        // Some(i) matches some and any i32 value
        Some(i) => Some(i + 1),
    }
}
