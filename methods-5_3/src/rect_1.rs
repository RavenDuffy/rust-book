#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl (implementation) allows us to associate things with the Rectangle struct
impl Rectangle {
    // the area function uses &self here instead of height and width (they're exposed by self)
    // &self is short for self: &Self (if we wanted to type the param this is what it would be)
    // &self can be mutable (with &mut self) and we can even take ownership (width self)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // this method shares the same name as the struct field however, we can still use both (using
    // brackets to signify the method or none for the field)
    fn width(&self) -> bool {
        self.width > 0
    }
}

pub fn rectangle_properties() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area() // here we can just call the impl area function
    );

    // calls the width method
    if rect.width() {
        println!("The rectangle's width is none zero; it is {}", rect.width); // gets the field
    }
}
