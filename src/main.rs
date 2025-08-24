use rand::seq::SliceRandom;
use rand::prelude::*;
use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Rank {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone)]
struct Card {
    suit: Suit,
    rank: Rank,
}

fn generate_deck() -> Vec<Card> {
    let suits = vec![
        Suit::Spades,
        Suit::Hearts,
        Suit::Diamonds,
        Suit::Clubs,
    ];

    let mut deck = Vec::new();

    for suit in suits {
        for n in 2..=10 {
            deck.push(Card { suit: suit.clone(), rank: Rank::Number(n) });
        }
        deck.push(Card { suit: suit.clone(), rank: Rank::Jack });
        deck.push(Card { suit: suit.clone(), rank: Rank::Queen });
        deck.push(Card { suit: suit.clone(), rank: Rank::King });
        deck.push(Card { suit: suit.clone(), rank: Rank::Ace });
    }

    deck
}

fn display_card(card: &Card) -> String {
    let rank = match &card.rank {
        Rank::Ace => "A".to_string(),
        Rank::Number(n) => n.to_string(),
        Rank::Jack => "J".to_string(),
        Rank::Queen => "Q".to_string(),
        Rank::King => "K".to_string(),
    };

    let (suit_symbol, color_code) = match card.suit {
        Suit::Spades => ("♠", "\x1b[37m"),
        Suit::Clubs => ("♣", "\x1b[37m"),
        Suit::Hearts => ("♥", "\x1b[31m"),
        Suit::Diamonds => ("♦", "\x1b[31m"),
    };

    format!("{}{}{}{}", color_code, rank, suit_symbol, "\x1b[0m")
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

    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);
    let mut hands = vec![Vec::new(); num_hands];
    for i in 0..total_needed {
        let card = deck.pop().unwrap();
        hands[i % num_hands].push(card);
    }

    hands
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    HighCard(Vec<Rank>),
    OnePair(Rank),
    TwoPair(Rank, Rank),
    ThreeOfAKind(Rank),
    Straight(Rank),
    Flush(Vec<Rank>),
    FullHouse(Rank, Rank),
    FourOfAKind(Rank),
    StraightFlush(Rank),
    RoyalFlush,
}

fn evaluate_hand(hand: &[Card]) -> HandRank {
    let mut ranks: Vec<Rank> = hand.iter().map(|c| c.rank.clone()).collect();
    let suits: Vec<Suit> = hand.iter().map(|c| c.suit.clone()).collect();

    ranks.sort_by(|a, b| b.cmp(a)); // Descending
    let is_flush = suits.iter().all(|s| s == &suits[0]);

    let is_straight = {
        let mut nums: Vec<u8> = ranks.iter().map(|r| rank_value(r)).collect();
        nums.sort_unstable();
        nums.dedup();
        nums.windows(5).any(|w| w[4] == w[0] + 4) || nums == vec![2, 3, 4, 5, 14] // Handle A-2-3-4-5
    };

    let mut counts = HashMap::new();
    for r in &ranks {
        *counts.entry(r).or_insert(0) += 1;
    }

    let mut count_vec: Vec<(&Rank, usize)> = counts.iter().map(|(k, v)| (*k, *v)).collect();
    count_vec.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.0.cmp(a.0)));

    match count_vec.as_slice() {
        [(r4, 4), (_r1, 1)] => HandRank::FourOfAKind((*r4).clone()),
        [(r3, 3), (r2, 2)] => HandRank::FullHouse((*r3).clone(), (*r2).clone()),
        _ if is_flush && is_straight && ranks.contains(&Rank::Ace) => HandRank::RoyalFlush,
        _ if is_flush && is_straight => HandRank::StraightFlush(ranks[0].clone()),
        [(r3, 3), ..] => HandRank::ThreeOfAKind((*r3).clone()),
        [(r2a, 2), (r2b, 2), ..] => HandRank::TwoPair((*r2a).clone(), (*r2b).clone()),
        [(r2, 2), ..] => HandRank::OnePair((*r2).clone()),
        _ if is_flush => HandRank::Flush(ranks.clone()),
        _ if is_straight => HandRank::Straight(ranks[0].clone()),
        _ => HandRank::HighCard(ranks.clone()),
    }
}

fn rank_value(rank: &Rank) -> u8 {
    match rank {
        Rank::Number(n) => *n,
        Rank::Jack => 11,
        Rank::Queen => 12,
        Rank::King => 13,
        Rank::Ace => 14,
    }
}

fn determine_winner(hands: &[Vec<Card>]) -> usize {
    let mut ranked_hands: Vec<(usize, HandRank)> = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i, evaluate_hand(hand)))
        .collect();

    ranked_hands.sort_by(|a, b| b.1.cmp(&a.1)); // Highest first
    ranked_hands[0].0 // Return the index of the best hand
}

fn main() {
    let mut deck = generate_deck();
    let hands = deal_hands(&mut deck, 5, 4);

    for (i, hand) in hands.iter().enumerate() {
        println!("Hand {}:", i + 1);
        for card in hand {
            print!("{} ", display_card(card));
        }
        let rank = evaluate_hand(hand);
        println!("\n→ Poker Rank: {:?}\n", rank);
    }

    let winner = determine_winner(&hands);
    println!("🏆 Hand {} wins!", winner + 1);
}