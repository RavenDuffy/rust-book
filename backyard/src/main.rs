use crate::garden::vegetables::Asparagus; // use Asparagus in this context
// can use because we export at both levels

pub mod garden; // this tells the compiler to look for garden in `src/garden.rs`

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}");
}
