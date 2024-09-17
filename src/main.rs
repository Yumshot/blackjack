use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
use macroquad::text::{ Font, TextParams, measure_text };
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Rank {
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
struct Card {
    rank: Rank,
    suit: Suit,
}

struct Deck(Vec<Card>);

struct PlayerHand(Vec<Card>);

struct DealerHand(Vec<Card>);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Menu,
    InProgress,
    PlayerTurn,
    DealerTurn,
    PlayerWon,
    DealerWon,
    Tie,
}

impl Deck {
    fn new() -> Self {
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
                cards.push(Card {
                    rank: *rank,
                    suit: *suit,
                });
            }
        }
        cards.shuffle();
        Deck(cards)
    }

    fn draw(&mut self) -> Option<Card> {
        self.0.pop()
    }
}

fn calculate_hand_value(hand: &[Card]) -> i32 {
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

    // Adjust for Aces being 1 or 11
    while value > 21 && aces > 0 {
        value -= 10;
        aces -= 1;
    }

    value
}

// Helper function to generate the image filename for a card
fn card_image_filename(card: Card) -> String {
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

    format!("assets\\images\\{}{}.png", rank_str, suit_str)
}

// Load all card images into a HashMap
async fn load_card_images() -> HashMap<String, Texture2D> {
    let mut images = HashMap::new();
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
            let card = Card { rank: *rank, suit: *suit };
            let filename = card_image_filename(card);

            match load_texture(&filename).await {
                Ok(texture) => {
                    images.insert(filename, texture);
                }
                Err(_) => {
                    eprintln!("Failed to load image: {}", filename); // Log the error
                }
            }
        }
    }
    images
}

fn draw_centered_text(text: &str, y: f32, font_size: u16, color: Color, font: Option<&Font>) {
    let text_dimensions = measure_text(text, font, font_size, 1.0);
    let x = (screen_width() - text_dimensions.width) / 2.0;
    draw_text_ex(text, x, y, TextParams {
        font,
        font_size,
        color,
        ..Default::default()
    });
}

// Draw a hand of cards using the card images
fn draw_hand(hand: &[Card], y: f32, images: &HashMap<String, Texture2D>, max_width: f32) {
    let card_width = 70.0;
    let total_width = card_width * (hand.len() as f32);
    let start_x = (max_width - total_width) / 2.0;

    for (i, card) in hand.iter().enumerate() {
        let filename = card_image_filename(*card);
        if let Some(texture) = images.get(&filename) {
            draw_texture(&*texture, start_x + (i as f32) * card_width, y, WHITE);
        }
    }
}

#[macroquad::main("Blackjack")]
async fn main() {
    let custom_font = load_ttf_font("assets\\fonts\\font.ttf").await.unwrap();
    let card_images = load_card_images().await; // Load the card images

    let mut game_state = GameState::Menu;
    let mut deck = Deck::new();
    let mut player_hand = PlayerHand(Vec::new());
    let mut dealer_hand = DealerHand(Vec::new());

    loop {
        clear_background(BLACK);

        // Get current screen size
        let screen_width = screen_width();
        let screen_height = screen_height();

        // Calculate dynamic font sizes based on screen height (responsive)
        let title_font_size = screen_height * 0.05;
        let normal_font_size = screen_height * 0.03;

        match game_state {
            GameState::Menu => {
                draw_centered_text(
                    "Welcome to Blackjack!",
                    screen_height * 0.3,
                    title_font_size as u16,
                    WHITE,
                    Some(&custom_font)
                );
                draw_centered_text(
                    "Press ENTER to start",
                    screen_height * 0.4,
                    normal_font_size as u16,
                    WHITE,
                    Some(&custom_font)
                );

                if is_key_pressed(KeyCode::Enter) {
                    for _ in 0..2 {
                        player_hand.0.push(deck.draw().unwrap());
                        dealer_hand.0.push(deck.draw().unwrap());
                    }
                    game_state = GameState::PlayerTurn;
                }
            }

            GameState::PlayerTurn => {
                let player_value = calculate_hand_value(&player_hand.0);

                // Display player's hand
                draw_hand(&player_hand.0, screen_height * 0.6, &card_images, screen_width);

                draw_centered_text(
                    &format!("Player Hand Value: {}", player_value),
                    screen_height * 0.5,
                    normal_font_size as u16,
                    WHITE,
                    Some(&custom_font)
                );
                draw_centered_text(
                    "Press SPACE to hit or ENTER to hold",
                    screen_height * 0.65,
                    normal_font_size as u16,
                    WHITE,
                    Some(&custom_font)
                );

                if player_value > 21 {
                    game_state = GameState::DealerWon;
                }

                if is_key_pressed(KeyCode::Space) {
                    if let Some(card) = deck.draw() {
                        player_hand.0.push(card);
                    }
                } else if is_key_pressed(KeyCode::Enter) {
                    game_state = GameState::DealerTurn;
                }
            }

            GameState::DealerTurn => {
                // Display dealer's hand
                draw_hand(&dealer_hand.0, screen_height * 0.3, &card_images, screen_width);

                while calculate_hand_value(&dealer_hand.0) < 17 {
                    if let Some(card) = deck.draw() {
                        dealer_hand.0.push(card);
                    }
                }

                let player_value = calculate_hand_value(&player_hand.0);
                let dealer_value = calculate_hand_value(&dealer_hand.0);

                if dealer_value > 21 || player_value > dealer_value {
                    game_state = GameState::PlayerWon;
                } else if dealer_value > player_value {
                    game_state = GameState::DealerWon;
                } else {
                    game_state = GameState::Tie;
                }
            }

            // Handle Win/Loss/Tie States
            GameState::PlayerWon => {
                draw_centered_text(
                    "Player Won!",
                    screen_height * 0.3,
                    title_font_size as u16,
                    WHITE,
                    Some(&custom_font)
                );
                draw_centered_text(
                    "Press R to restart",
                    screen_height * 0.4,
                    normal_font_size as u16,
                    WHITE,
                    Some(&custom_font)
                );
                if is_key_pressed(KeyCode::R) {
                    deck = Deck::new();
                    player_hand.0.clear();
                    dealer_hand.0.clear();
                    game_state = GameState::Menu;
                }
            }

            GameState::DealerWon => {
                draw_centered_text(
                    "Dealer Won!",
                    screen_height * 0.3,
                    title_font_size as u16,
                    WHITE,
                    Some(&custom_font)
                );
                draw_centered_text(
                    "Press R to restart",
                    screen_height * 0.4,
                    normal_font_size as u16,
                    WHITE,
                    Some(&custom_font)
                );
                if is_key_pressed(KeyCode::R) {
                    deck = Deck::new();
                    player_hand.0.clear();
                    dealer_hand.0.clear();
                    game_state = GameState::Menu;
                }
            }

            GameState::Tie => {
                draw_centered_text(
                    "It's a Tie!",
                    screen_height * 0.3,
                    title_font_size as u16,
                    WHITE,
                    Some(&custom_font)
                );
                draw_centered_text(
                    "Press R to restart",
                    screen_height * 0.4,
                    normal_font_size as u16,
                    WHITE,
                    Some(&custom_font)
                );
                if is_key_pressed(KeyCode::R) {
                    deck = Deck::new();
                    player_hand.0.clear();
                    dealer_hand.0.clear();
                    game_state = GameState::Menu;
                }
            }
            GameState::InProgress => {}
        }

        next_frame().await;
    }
}
