mod cards;
mod game;
mod ui;

use macroquad::prelude::*;
use crate::ui::{ draw_centered_text, draw_hand, load_card_images };
use crate::game::{ Game, GameState };

#[macroquad::main("Blackjack")]
async fn main() {
    let custom_font = load_ttf_font("assets/fonts/font.ttf").await.unwrap();
    let card_images = load_card_images().await;

    let mut game = Game::new();

    loop {
        clear_background(BLACK);
        let screen_width = screen_width();
        let screen_height = screen_height();

        let title_font_size = screen_height * 0.05;
        let normal_font_size = screen_height * 0.03;

        match game.game_state {
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
                        game.player_hit();
                        game.dealer_hit(); // Added dealer cards
                    }
                    game.game_state = GameState::PlayerTurn;
                }
            }

            GameState::PlayerTurn => {
                draw_hand(
                    &game.dealer_hand.0,
                    screen_height * 0.4,
                    &card_images,
                    screen_width,
                    false
                ).await;
                draw_hand(
                    &game.player_hand.0,
                    screen_height * 0.7,
                    &card_images,
                    screen_width,
                    true
                ).await;

                let player_value = game.calculate_player_value(); // Now the method is implemented

                draw_centered_text(
                    &format!("Player Hand Value: {}", player_value),
                    screen_height * 0.65,
                    normal_font_size as u16,
                    WHITE,
                    Some(&custom_font)
                );
                draw_centered_text(
                    "Press SPACE to hit or ENTER to hold",
                    screen_height * 0.95,
                    normal_font_size as u16,
                    WHITE,
                    Some(&custom_font)
                );

                if player_value > 21 {
                    game.game_state = GameState::DealerWon;
                }

                if is_key_pressed(KeyCode::Space) {
                    game.player_hit();
                } else if is_key_pressed(KeyCode::Enter) {
                    game.game_state = GameState::DealerTurn;
                }
            }

            GameState::DealerTurn => {
                draw_hand(
                    &game.dealer_hand.0,
                    screen_height * 0.3,
                    &card_images,
                    screen_width,
                    true
                ).await;
                draw_hand(
                    &game.player_hand.0,
                    screen_height * 0.6,
                    &card_images,
                    screen_width,
                    true
                ).await;

                while game.calculate_dealer_value() < 17 {
                    game.dealer_hit(); // Dealer keeps drawing until reaching 17 or more
                }

                let player_value = game.calculate_player_value();
                let dealer_value = game.calculate_dealer_value();

                if dealer_value > 21 || player_value > dealer_value {
                    game.game_state = GameState::PlayerWon;
                } else if dealer_value > player_value {
                    game.game_state = GameState::DealerWon;
                } else {
                    game.game_state = GameState::Tie;
                }
            }

            GameState::PlayerWon => {
                draw_hand(
                    &game.dealer_hand.0,
                    screen_height * 0.3,
                    &card_images,
                    screen_width,
                    true
                ).await;
                draw_hand(
                    &game.player_hand.0,
                    screen_height * 0.6,
                    &card_images,
                    screen_width,
                    true
                ).await;

                draw_centered_text(
                    "Player Won!",
                    screen_height * 0.25,
                    title_font_size as u16,
                    WHITE,
                    Some(&custom_font)
                );
                draw_centered_text(
                    "Press R to restart",
                    screen_height * 0.5,
                    normal_font_size as u16,
                    WHITE,
                    Some(&custom_font)
                );
                if is_key_pressed(KeyCode::R) {
                    game.reset();
                    game.game_state = GameState::Menu;
                }
            }

            GameState::DealerWon => {
                draw_hand(
                    &game.dealer_hand.0,
                    screen_height * 0.3,
                    &card_images,
                    screen_width,
                    true
                ).await;
                draw_hand(
                    &game.player_hand.0,
                    screen_height * 0.6,
                    &card_images,
                    screen_width,
                    true
                ).await;

                draw_centered_text(
                    "Dealer Won!",
                    screen_height * 0.25,
                    title_font_size as u16,
                    WHITE,
                    Some(&custom_font)
                );
                draw_centered_text(
                    "Press R to restart",
                    screen_height * 0.5,
                    normal_font_size as u16,
                    WHITE,
                    Some(&custom_font)
                );
                if is_key_pressed(KeyCode::R) {
                    game.reset();
                    game.game_state = GameState::Menu;
                }
            }

            GameState::Tie => {
                draw_hand(
                    &game.dealer_hand.0,
                    screen_height * 0.3,
                    &card_images,
                    screen_width,
                    true
                ).await;
                draw_hand(
                    &game.player_hand.0,
                    screen_height * 0.6,
                    &card_images,
                    screen_width,
                    true
                ).await;

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
                    game.reset();
                    game.game_state = GameState::Menu;
                }
            }
        }

        next_frame().await;
    }
}
