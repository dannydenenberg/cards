mod cards;

mod art;

// when you want absolute path, use `use crate::stuff::whatevs`



extern crate termion;

use cards::cards::{Player, Deck, Card};

fn main() {

    // testing
    let mut p = Player { hand: vec![Card::Spade("5".to_string()), Card::Club("K".to_string()), Card::Heart("10".to_string())], name: "Jerry".to_string()};
    p.print_hand();

}




