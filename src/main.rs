mod play;

mod art;

// when you want absolute path, use `use crate::stuff::whatevs`



extern crate termion;

use crate::play::{Player, Deck};


fn main() {
    let mut d = play::Deck::new();
    d.shuffle();
    d.print();

    let mut danny = Player::new();

    for _ in 0..10 {
        match danny.pick_card(&mut d) {
            Ok(card) => danny.put_card_in_hand(card),
            Err(x) => (),
        }
    }

    danny.print_hand();
}




