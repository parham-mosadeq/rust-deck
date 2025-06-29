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
}

fn main() {
    let deck = Deck::new_deck();
    let heart_deck = deck.get_hearts();
    let diamond_deck = deck.get_diamonds();
    println!("Hearts: {:#?}", heart_deck);
    println!("Diamonds: {:#?}", diamond_deck);
}
