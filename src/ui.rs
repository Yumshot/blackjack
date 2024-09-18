use macroquad::prelude::*;
use macroquad::text::{ Font, TextParams, measure_text };
use std::collections::HashMap;
use crate::cards::{ card_image_filename, Card, Rank, Suit };

pub async fn load_card_images() -> HashMap<String, Texture2D> {
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
            if let Ok(texture) = load_texture(&filename).await {
                images.insert(filename, texture);
            }
        }
    }
    images
}

pub fn draw_centered_text(text: &str, y: f32, font_size: u16, color: Color, font: Option<&Font>) {
    let text_dimensions = measure_text(text, font, font_size, 1.0);
    let x = (screen_width() - text_dimensions.width) / 2.0;
    draw_text_ex(text, x, y, TextParams {
        font,
        font_size,
        color,
        ..Default::default()
    });
}

pub async fn draw_hand(
    hand: &[Card],
    y: f32,
    images: &HashMap<String, Texture2D>,
    max_width: f32,
    show_all: bool
) {
    let card_width = 70.0;
    let card_height = 100.0;
    let total_width = card_width * (hand.len() as f32);
    let start_x = (max_width - total_width) / 2.0;

    let cards_to_show = if show_all { hand.len() } else { 1 };

    for (i, card) in hand.iter().take(cards_to_show).enumerate() {
        let filename = card_image_filename(*card);
        if let Some(texture) = images.get(&filename) {
            draw_texture_ex(
                &*texture,
                start_x + (i as f32) * card_width,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(card_width, card_height)),
                    ..Default::default()
                }
            );
        }
    }

    if !show_all && hand.len() > 1 {
        let hidden_card_texture = load_texture("assets/images/cardback.png").await.unwrap();
        draw_texture_ex(&hidden_card_texture, start_x + card_width, y, WHITE, DrawTextureParams {
            dest_size: Some(vec2(card_width, card_height)),
            ..Default::default()
        });
    }
}
