fn main() {}

mod back_of_house {
    pub enum Appetiser {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // to access these orders we don't need to make each enum field public
    let order1 = back_of_house::Appetiser::Soup;
    let order2 = back_of_house::Appetiser::Salad;
}
