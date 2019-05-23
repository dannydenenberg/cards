mod play;

mod art;

// when you want absolute path, use `use crate::stuff::whatevs`



extern crate termion;


fn main() {
    let mut d = play::Deck::new();
    d.print();
    println!("\nSHUFFLE:\n\n");
    d.shuffle();
    d.print();
}




