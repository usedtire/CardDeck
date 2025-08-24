use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::prelude::IndexedRandom;


#[derive(Debug, Clone)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Debug, Clone)]
enum Rank {
    Ace,
    Number(u8), // 2 through 10
    Jack,
    Queen,
    King,
}

#[derive(Debug, Clone)]
struct Card {
    suit: Suit,
    rank: Rank,
}

// Function to generate a full 52-card deck
fn generate_deck() -> Vec<Card> {
    let suits = vec![
        Suit::Spades,
        Suit::Hearts,
        Suit::Diamonds,
        Suit::Clubs,
    ];

    let mut deck = Vec::new();

    for suit in suits {
        // Add Ace
        deck.push(Card { suit: suit.clone(), rank: Rank::Ace });

        // Add 2 through 10
        for n in 2..=10 {
            deck.push(Card { suit: suit.clone(), rank: Rank::Number(n) });
        }

        // Add Jack, Queen, King
        deck.push(Card { suit: suit.clone(), rank: Rank::Jack });
        deck.push(Card { suit: suit.clone(), rank: Rank::Queen });
        deck.push(Card { suit: suit.clone(), rank: Rank::King });
    }

    deck
}

// Helper function to display a card nicely
fn display_card(card: &Card) -> String {
    let rank = match card.rank {
        Rank::Ace => "A".to_string(),
        Rank::Number(n) => n.to_string(),
        Rank::Jack => "J".to_string(),
        Rank::Queen => "Q".to_string(),
        Rank::King => "K".to_string(),
    };

    let suit = match card.suit {
        Suit::Spades => "♠",
        Suit::Hearts => "♥",
        Suit::Diamonds => "♦",
        Suit::Clubs => "♣",
    };

    format!("{}{}", rank, suit)
}
fn deal_hands(deck: &mut Vec<Card>, cards_per_hand: usize, num_hands: usize) -> Vec<Vec<Card>> {
    let total_needed = cards_per_hand * num_hands;
    if total_needed > deck.len() {
        panic!(
            "Not enough cards in deck! Requested {} but only {} available.",
            total_needed,
            deck.len()
        );
    }

    deck.shuffle(&mut thread_rng());

    let mut hands = vec![Vec::new(); num_hands];

    for i in 0..total_needed {
        let card = deck.pop().unwrap();
        hands[i % num_hands].push(card);
    }

    hands
}

fn main() {
    let mut deck = generate_deck();

    let cards_per_hand = 5;
    let num_hands = 4;

    let hands = deal_hands(&mut deck, cards_per_hand, num_hands);

    for (i, hand) in hands.iter().enumerate() {
        println!("Hand {}:", i + 1);
        for card in hand {
            print!("{} ", display_card(card));
        }
        println!("\n");
    }

}