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
}

fn main() {
    let deck = Deck::new_deck();

    println!("Hello, world! {:#?}", deck);
}
