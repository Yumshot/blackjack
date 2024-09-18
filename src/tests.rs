#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_load_card_images() {
        let start = Instant::now();

        let images = load_card_images().await;

        let duration = start.elapsed();
        println!("test_load_card_images took: {:?}", duration);

        assert_eq!(images.len(), 52);
    }

    #[test]
    fn test_card_image_filename() {
        let card = Card {
            rank: Rank::Ace,
            suit: Suit::Hearts,
        };
        let filename = card_image_filename(card);
        assert_eq!(filename, "assets/images/Ace_of_Hearts.png");
    }

    #[tokio::test]
    async fn test_draw_centered_text() {
        let mut app = App::new();
        let font = load_ttf_font("assets/fonts/font.ttf").await.unwrap();
        draw_centered_text("Test", 100.0, 30, WHITE, Some(&font));
        app.run().await;
    }

    #[tokio::test]
    async fn test_draw_hand() {
        let mut app = App::new();
        let images = load_card_images().await;
        let hand = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Hearts,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Diamonds,
            }
        ];
        draw_hand(&hand, 100.0, &images, 200.0, true).await;
        app.run().await;
    }
}
