use rand::seq::SliceRandom;
use std::{collections::HashMap, fmt, io};
use strfmt::strfmt;

struct Card {
    value: String,
    suit: String,
}

fn main() {
    init_deck();
}

fn init_deck() -> Vec<Vec<Card>> {
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
    let card_map = HashMap::new();
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
    for suit in suits {
        for card in cards {}
    }
}
