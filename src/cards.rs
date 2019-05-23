




/********************************* OLD: ***********************************/

pub mod cards {
    extern crate rand;
    use rand::{thread_rng, Rng};
    use crate::art::*; // club, diamond, spade, heart

    //use super::rand::distributions::Open01;

    // represents a single card
    pub enum Card {
        Heart(String),
        Spade(String),
        Club(String),
        Diamond(String),
    }

    // Represents a deck of 52 cards
    pub struct Deck {
        cards: Vec<Card>
    }

    impl Deck {
        // constructs a new deck with 52 `Card`s
        pub fn new() -> Deck {
            let vals = vec!["A".to_string(),"1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),"5".to_string(),"6".to_string(),"7".to_string(),"8".to_string(),"9".to_string(),"10".to_string(),"J".to_string(),"Q".to_string(),"K".to_string()];
            let mut list= vec![];

            for i in vals {
                list.push(Card::Club(i.clone()));
                list.push(Card::Heart(i.clone()));
                list.push(Card::Spade(i.clone()));
                list.push(Card::Diamond(i.clone()));
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
            if self.cards.len() == 0 { return None; }
            Some(self.cards.pop().unwrap())
        }

        pub fn print(&self) {
            for i in &self.cards {
                match i {
                    Card::Diamond(x) => println!("{}💎", x),
                    Card::Heart(x) => println!("{}❤️", x),
                    Card::Club(x) => println!("{}♧", x),
                    Card::Spade(x) => println!("{}♠", x),
                }
            }
        }
    }



    trait Game {
        fn is_next_turn() -> bool;
        fn next_turn();
        fn take_turn();
    }



    pub struct Player {
        pub hand: Vec<Card>,
        pub name: String,
    }

    impl Player {
        pub fn new() -> Self {
            // TODO: FINISH THIS
            Player { hand: Vec::new(), name: "Jerry".to_string() }
        }

        // private helper function
        fn pick_from_deck(&mut self, deck: &mut Deck) -> Option<Card> {
            deck.pick()
        }

        // private helper function
        fn put_card_in_hand(&mut self, card: Card) {
            self.hand.push(card);
        }

        // public function to deal with what happens when a card is picked
        pub fn pick_card(&mut self, deck: &mut Deck)  {
            // TODO: FINISH THIS
            // if Some()
        }

        pub fn print_hand(&self) {
            let mut cards_in_hand: Vec<String> = Vec::new();

            for card in &self.hand {
                cards_in_hand.push(match card {
                    Card::Diamond(c) => diamond.replace("#", c).to_string(),
                    Card::Heart(c) => heart.replace("#", c).to_string(),
                    Card::Club(c) => club.replace("#", c).to_string(),
                    Card::Spade(c) => spade.replace("#", c).to_string(),
                });
            }

            print_side_by_side(cards_in_hand);
        }
    }



    // takes same dimension ascii drawings and prints them side by side
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

}