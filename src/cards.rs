use rand::seq::SliceRandom; // Import SliceRandom for shuffling
use rand::thread_rng; // Use thread_rng to generate random numbers

#[derive(Debug, Clone, Copy)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

pub struct Deck(pub Vec<Card>);
pub struct PlayerHand(pub Vec<Card>);
pub struct DealerHand(pub Vec<Card>);

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in &[
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
                Rank::Ace,
            ] {
                cards.push(Card { rank: *rank, suit: *suit });
            }
        }

        // Shuffle the deck using rand crate
        let mut rng = thread_rng();
        cards.shuffle(&mut rng); // Shuffle using thread_rng for randomness

        Deck(cards)
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.0.pop()
    }
}

pub fn calculate_hand_value(hand: &[Card]) -> i32 {
    let mut value = 0;
    let mut aces = 0;

    for card in hand {
        value += match card.rank {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
            Rank::Ace => 11,
        };

        if card.rank == Rank::Ace {
            aces += 1;
        }
    }

    while value > 21 && aces > 0 {
        value -= 10;
        aces -= 1;
    }

    value
}

pub fn card_image_filename(card: Card) -> String {
    let rank_str = match card.rank {
        Rank::Two => "2",
        Rank::Three => "3",
        Rank::Four => "4",
        Rank::Five => "5",
        Rank::Six => "6",
        Rank::Seven => "7",
        Rank::Eight => "8",
        Rank::Nine => "9",
        Rank::Ten => "10",
        Rank::Jack => "jack",
        Rank::Queen => "queen",
        Rank::King => "king",
        Rank::Ace => "ace",
    };

    let suit_str = match card.suit {
        Suit::Hearts => "hearts",
        Suit::Diamonds => "diamonds",
        Suit::Clubs => "clubs",
        Suit::Spades => "spades",
    };

    format!("assets/images/{}{}.png", rank_str, suit_str)
}
