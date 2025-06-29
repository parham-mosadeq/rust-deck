use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new_deck() -> Self {
        // * Lists of suits
        let suits = vec!["Hearts", "Diamonds", "Clubs", "Spades"];
        // * Lists of ranks
        let ranks = vec![
            "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace",
        ];

        // * Mut to be able to change it (push to it)
        let mut cards = vec![];

        for suit in &suits {
            for rank in &ranks {
                let card = format!("{rank} of {suit}");
                cards.push(card);
            }
        }

        let deck = Deck { cards };
        deck
    }

    fn get_hearts(&self) -> Deck {
        let cloned = self.cards.clone();
        let mut cards = vec![];
        for hearts in cloned {
            if hearts.contains("Hearts") {
                cards.push(hearts);
            }
        }
        Deck { cards }
    }

    fn get_diamonds(&self) -> Deck {
        let cloned = self.cards.clone();
        let mut cards = vec![];

        for diamond in cloned {
            if diamond.contains("Diamonds") {
                cards.push(diamond);
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rngs = rng();
        self.cards.shuffle(&mut rngs);
    }

    fn deal(&mut self, split_num: usize) -> Deck {
        let cards = self.cards.split_off(split_num);
        Deck { cards }
    }
}

fn main() {
    let mut deck = Deck::new_deck();
    let mut heart_deck = deck.get_hearts();
    // let diamond_deck = deck.get_diamonds();
    deck.shuffle();
    heart_deck.shuffle();
    deck.deal(5);
    println!("Here is Shuffled Deck: {:#?}", deck);
    // println!("Here is Shuffled Hearts Deck: {:#?}", heart_deck);
}
