extern crate rand;
use rand::{thread_rng, Rng};

use crate::art::*;


pub struct Card {
    suit: Suit,
    value: String,
}

enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    // constructs a new deck with 52 `Card`s
    pub fn new() -> Deck {
        let vals = vec!["A".to_string(),"1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),"5".to_string(),"6".to_string(),"7".to_string(),"8".to_string(),"9".to_string(),"10".to_string(),"J".to_string(),"Q".to_string(),"K".to_string()];
        let mut list= vec![];

        for i in vals {
            list.push(Card { suit: Suit::Heart, value: i.clone()});
            list.push(Card { suit: Suit::Diamond, value: i.clone()});
            list.push(Card { suit: Suit::Club, value: i.clone()});
            list.push(Card { suit: Suit::Spade, value: i.clone()});
        }

        Deck { cards: list }
    }

    // realllly shuffles a deck!
    pub fn shuffle(&mut self) {
        for i in 0..self.cards.len() {
            let index = rand::thread_rng().gen_range(0,  self.cards.len()) as usize;
            self.cards.swap(i, index);
        }
    }

    pub fn pick(&mut self) -> Option<Card> {
        if self.cards.len() == 0 { return None }
        self.cards.pop() // the pop function returns an option
    }

    pub fn print(&self) {
        for i in &self.cards {
            match &i {
                &Card{ suit: Suit::Diamond, value: s} => println!("{}💎", s),
                &Card{ suit: Suit::Heart, value: s} => println!("{}❤", s),
                &Card{ suit: Suit::Club, value: s} => println!("{}♧", s),
                &Card{ suit: Suit::Spade, value: s} => println!("{}♠", s),
                _ => (),
            }
        }
    }
}

pub struct Player {
    hand: Vec<Card>,
}

impl Player {
    pub fn new() -> Self {
        Player { hand: Vec::new(), }
    }

    // private helper function
    pub fn put_card_in_hand(&mut self, card: Card) {
        self.hand.push(card);
    }

    /// Attempts to pick a card from the supplied deck.
    pub fn pick_card(&mut self, deck: &mut Deck) -> Result<Card, bool> {
        match deck.pick() {
            Some(c) => Ok(c),
            None => Err(true),
        }
    }

    pub fn print_hand(&self) {
        let mut cards_in_hand: Vec<String> = Vec::new();

        for card in &self.hand {
            cards_in_hand.push(match card {
                // the 10's have to be printed differently because it is the only card with two characters as its value
                Card { suit: Suit::Diamond, value: v } => {
                    if v == &"10".to_string() {
                        diamond10.replace("#", v).to_string()
                    } else {
                        diamond.replace("#", v).to_string()
                    }
                },
                Card { suit: Suit::Club, value: v } => {
                    if v == &"10".to_string() {
                        club10.replace("#", v).to_string()
                    } else {
                        club.replace("#", v).to_string()
                    }
                },
                Card { suit: Suit::Heart, value: v } => {
                    if v == &"10".to_string() {
                        heart10.replace("#", v).to_string()
                    } else {
                        heart.replace("#", v).to_string()
                    }
                },
                Card { suit: Suit::Spade, value: v } => {
                    if v == &"10".to_string() {
                        spade10.replace("#", v).to_string()
                    } else {
                        spade.replace("#", v).to_string()
                    }
                },
                _ => "".to_string(), // won't reach this statement, but the compiler doesn't know that
            });
        }

        print_side_by_side(cards_in_hand);
    }
}



/// takes same dimension ascii drawings and prints them side by side
fn print_side_by_side(drawings: Vec<String>) {
    // represents the lines of each drawing
    let mut lines: Vec<Vec<String>> = Vec::new();

    // go through the drawings, and put each line into its own vector in the lines vector
    for pic in drawings {
        lines.push(pic.split("\n").collect::<Vec<&str>>().iter().map(|s| s.to_string()).collect());
    }

    //// ***NOTE: this assumes all of the drawings are of the same dimentions!***
    for i in 0..lines[0].clone().len() {
        // go through all of the pictures and print each line next to eachother
        for k in lines.clone() {
            print!("{}", k[i]);
        }
        println!();
    }
}