use rand::{seq::SliceRandom, thread_rng};
use std::{collections::HashMap, fmt, io};
use strfmt::strfmt;

#[derive(Debug, Clone)]
struct Card {
    value: String,
    suit: String,
    pub flipped_over: bool,
}

fn main() {
    let mut deck = create_deck();
    let mut wallet = 1000.0;
    let mut player_hand: Vec<Card> = Vec::new();
    let mut dealer_hand: Vec<Card> = Vec::new();

    deal(&mut deck, &mut player_hand, &mut dealer_hand);

    for card in dealer_hand.iter() {
        let str = create_card_string(&card);
        println!("{}", str);
    }
}

fn display_frame(player_hand: &Vec<Card>, dealer_hand: &Vec<Card>, wallet: &f64) {
    let frame = String::new();
    for player_card in player_hand.iter() {
        let card_string = create_card_string(player_card);
    }
}

fn deal(deck: &mut Vec<Card>, player_hand: &mut Vec<Card>, dealer_hand: &mut Vec<Card>) {
    player_hand.push(deck[0].clone());
    deck.rotate_right(1);
    player_hand.push(deck[0].clone());
    deck.rotate_right(1);

    player_hand[0].flipped_over = false;
    player_hand[1].flipped_over = false;

    dealer_hand.push(deck[0].clone());
    deck.rotate_right(1);
    dealer_hand.push(deck[0].clone());
    deck.rotate_right(1);

    dealer_hand[0].flipped_over = false;
    dealer_hand[1].flipped_over = true;
}

fn create_deck() -> Vec<Card> {
    let blank = {
        "┌─────────┐
│░░░░░░░░░│
│░░░░░░░░░│
│░░░░░░░░░│
│░░░░░░░░░│
│░░░░░░░░░│
│░░░░░░░░░│
│░░░░░░░░░│
└─────────┘"
    };
    let card_string = {
        "┌─────────┐
│{num}        │
│         │
│         │
│    {suit}    │
│         │
│         │
│       {num} │
└─────────┘"
    };
    let ten_card_string = {
        "┌─────────┐
│{num}       │
│         │
│         │
│    {suit}   │
│         │
│         │
│       {num}│
└─────────┘"
    };
    // let mut vars = HashMap::new();
    // vars.insert("num".to_string(), "5");
    // vars.insert("suit".to_string(), "♠");
    // println!("{}", strfmt(&card_string, &vars).unwrap());
    let mut card_map = HashMap::new();
    card_map.insert("2".to_string(), 2);
    card_map.insert("3".to_string(), 3);
    card_map.insert("4".to_string(), 4);
    card_map.insert("5".to_string(), 5);
    card_map.insert("6".to_string(), 6);
    card_map.insert("7".to_string(), 7);
    card_map.insert("8".to_string(), 8);
    card_map.insert("9".to_string(), 9);
    card_map.insert("10".to_string(), 10);
    card_map.insert("J".to_string(), 10);
    card_map.insert("Q".to_string(), 10);
    card_map.insert("K".to_string(), 10);
    card_map.insert("A".to_string(), 11);

    let suits = ["♠", "♦", "♥", "♣"];
    let cards = [
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
    ];
    let mut deck: Vec<Card> = Vec::new();
    for _ in 0..4 {
        for suit in suits {
            for card in cards {
                deck.push(Card {
                    value: card.to_string(),
                    suit: suit.to_string(),
                    flipped_over: false,
                });
            }
        }
    }
    deck.shuffle(&mut thread_rng());
    return deck;
}

fn create_card_string(card: &Card) -> String {
    let blank = {
        "┌─────────┐
│░░░░░░░░░│
│░░░░░░░░░│
│░░░░░░░░░│
│░░░░░░░░░│
│░░░░░░░░░│
│░░░░░░░░░│
│░░░░░░░░░│
└─────────┘"
    };
    let card_string = {
        "┌─────────┐
│{num}        │
│         │
│         │
│    {suit}    │
│         │
│         │
│       {num} │
└─────────┘"
    };
    let ten_card_string = {
        "┌─────────┐
│{num}       │
│         │
│         │
│    {suit}    │
│         │
│         │
│       {num}│
└─────────┘"
    };
    if card.flipped_over {
        return blank.to_string();
    }

    let mut vars = HashMap::new();
    vars.insert("suit".to_string(), card.suit.clone());
    vars.insert("num".to_string(), card.value.clone());

    if card.value == "10" {
        return strfmt(&ten_card_string, &vars).unwrap();
    }

    return strfmt(&card_string, &vars).unwrap();
}
