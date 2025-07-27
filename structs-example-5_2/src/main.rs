#[derive(Debug)] // this allows the Rectangle struct to implement the Debug trait
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}"); // variable:? in format strings allows rust to use the Debug
    // trait, adding # (variable:#?) will make the style the output

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale), // dbg returns ownership so we can wrap our fields with it
        height: 50,
    };

    dbg!(&rect2); // dbg auto invokes the Debug print trait, dbg takes ownership unlike println
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
