extern crate rand;
use rand::{thread_rng, Rng};

struct Card {
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
        return self.cards.pop(); // the pop function returns an option
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
    fn pick_from_deck(&mut self, deck: &mut Deck) -> Option<Card> {
        deck.pick()
    }
    
}