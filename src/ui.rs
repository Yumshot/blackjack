use macroquad::prelude::*;
use macroquad::text::{ Font, TextParams, measure_text };
use tokio::sync::Mutex;
use crate::cards::{ card_image_filename, Card, Rank, Suit };
use std::collections::HashMap;
use std::sync::Arc;
use futures::future::join_all;
use lazy_static::lazy_static;

lazy_static! {
    static ref TEXTURE_CACHE: Arc<Mutex<HashMap<String, Texture2D>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub async fn load_card_images() -> HashMap<String, Texture2D> {
    let mut futures = Vec::new();

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

            // Push the future for loading the texture into the futures vector
            futures.push(load_or_get_cached_texture(filename.clone()));
        }
    }

    // Await all the futures in parallel
    let results = join_all(futures).await;

    // Collect the results into the HashMap
    results
        .into_iter()
        .filter_map(|result| result) // Filter out None values
        .collect()
}

// This function either loads the texture or fetches it from the cache
async fn load_or_get_cached_texture(filename: String) -> Option<(String, Texture2D)> {
    let cache = TEXTURE_CACHE.clone();
    let mut cache_lock = cache.lock().await;

    if let Some(texture) = cache_lock.get(&filename) {
        // Return cached texture
        return Some((filename, texture.clone()));
    }

    // If not cached, load the texture and insert it into the cache
    if let Ok(texture) = load_texture(&filename).await {
        cache_lock.insert(filename.clone(), texture.clone());
        Some((filename, texture))
    } else {
        None
    }
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
