use clearscreen;
use crossterm;
use rand::{seq::SliceRandom, thread_rng};
use std::{collections::HashMap, fmt, io, thread, time};
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
    display_frame(&player_hand, &dealer_hand, &wallet);
}

fn display_frame(player_hand: &Vec<Card>, dealer_hand: &Vec<Card>, wallet: &f64) {
    clearscreen::clear().expect("failed to clear screen");
    let mut frame: Vec<Vec<char>> = Vec::new();

    let mut row = 0;
    'row: while row < 9 {
        let mut card_count = 0;
        let mut row_vec: Vec<char> = Vec::new();

        'cards: for dealer_card in dealer_hand {
            let card_string = create_card_string(dealer_card);

            let mut num_lines = 0;
            for chr in card_string.chars() {
                if num_lines == row {
                    if chr == '\n' {
                        if card_count == dealer_hand.len() - 1 {
                            row_vec.push('\n');
                            num_lines += 1;
                            break 'cards;
                        }
                        num_lines += 1;
                        continue;
                    }
                    row_vec.push(chr);
                }
                if chr == '\n' {
                    num_lines += 1;
                }
            }
            card_count += 1;
        }
        frame.push(row_vec);
        row += 1;
    }

    let tmp: Vec<char> = vec!['\n', '\n', '\n'];
    frame.push(tmp);

    let mut row = 0;
    'row: while row < 9 {
        let mut card_count = 0;
        let mut row_vec: Vec<char> = Vec::new();

        'cards: for player_card in player_hand {
            let card_string = create_card_string(player_card);

            let mut num_lines = 0;
            for chr in card_string.chars() {
                if num_lines == row {
                    if chr == '\n' {
                        if card_count == player_hand.len() - 1 {
                            row_vec.push('\n');
                            num_lines += 1;
                            break 'cards;
                        }
                        num_lines += 1;
                        continue;
                    }
                    row_vec.push(chr);
                }
                if chr == '\n' {
                    num_lines += 1;
                }
            }
            card_count += 1;
        }
        frame.push(row_vec);
        row += 1;
    }

    for row in frame.iter() {
        for chr in row {
            print!("{}", chr);
        }
    }
    print!("       ${}\n", wallet);
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
