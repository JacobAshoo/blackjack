use rand::{seq::SliceRandom, thread_rng};
use std::{collections::HashMap, fmt, io};
use strfmt::strfmt;
use vector2d::Vector2D;

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
    display_frame(&player_hand, &dealer_hand, &wallet);
}

fn display_frame(player_hand: &Vec<Card>, dealer_hand: &Vec<Card>, wallet: &f64) {
    let mut frame: Vec<Vec<char>> = Vec::new();

    let mut row = 0;
    'row: while row < 9 {
        let mut card_count = 0;
        let mut row_vec: Vec<char> = Vec::new();

        for dealer_card in dealer_hand {
            let mut card_string = create_card_string(dealer_card);

            for chr in card_string.chars() {
                if chr == '\n' {
                    if card_count == dealer_hand.len() {
                        row_vec.push(chr);
                        //remove the row
                        card_string =
                            card_string[card_string.find("\n").unwrap_or(card_string.len())
                                ..card_string.len()]
                                .to_string();
                        break 'row;
                    }
                    row_vec.push(chr);
                }
                row_vec.push(' ');
                card_count += 1;
            }
        }
        frame.push(row_vec);
        row += 1;
    }
    for row in frame.iter() {
        for chr in row {
            print!("{}", chr);
        }
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
    // let mut vars = HashMap::new();
    // vars.insert("num".to_string(), "5");
    // vars.insert("suit".to_string(), "♠");
    // println!("{}", strfmt(&card_string, &vars).unwrap());

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
