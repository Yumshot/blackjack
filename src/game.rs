use crate::cards::{ Deck, PlayerHand, DealerHand, calculate_hand_value };

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Menu,
    PlayerTurn,
    DealerTurn,
    PlayerWon,
    DealerWon,
    Tie,
}

pub struct Game {
    pub game_state: GameState,
    pub deck: Deck,
    pub player_hand: PlayerHand,
    pub dealer_hand: DealerHand,
}

impl Game {
    pub fn new() -> Self {
        Self {
            game_state: GameState::Menu,
            deck: Deck::new(),
            player_hand: PlayerHand(Vec::new()),
            dealer_hand: DealerHand(Vec::new()),
        }
    }

    pub fn player_hit(&mut self) {
        if let Some(card) = self.deck.draw() {
            self.player_hand.0.push(card);
        }
    }

    pub fn dealer_hit(&mut self) {
        if let Some(card) = self.deck.draw() {
            self.dealer_hand.0.push(card);
        }
    }

    pub fn calculate_player_value(&self) -> i32 {
        calculate_hand_value(&self.player_hand.0)
    }

    pub fn calculate_dealer_value(&self) -> i32 {
        calculate_hand_value(&self.dealer_hand.0)
    }

    pub fn reset(&mut self) {
        self.deck = Deck::new();
        self.player_hand.0.clear();
        self.dealer_hand.0.clear();
    }
}
