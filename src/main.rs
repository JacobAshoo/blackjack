use clearscreen;
use device_query::{DeviceQuery, DeviceState, Keycode};
use rand::{seq::SliceRandom, thread_rng};
use spin_sleep;
use std::{collections::HashMap, io, io::Write, time::Duration};
use strfmt::strfmt;

#[derive(Debug, Clone)]
struct Card {
    value: String,
    suit: String,
    flipped_over: bool,
}

fn main() {
    let mut deck = create_deck();
    let mut wallet = 1000.0;
    let _spin_sleeper = spin_sleep::SpinSleeper::new(100_000)
        .with_spin_strategy(spin_sleep::SpinStrategy::YieldThread);

    'main: loop {
        let mut player_hand: Vec<Card> = Vec::new();
        let mut dealer_hand: Vec<Card> = Vec::new();

        //make bet
        clear_screan();
        println!("wallet: ${}", wallet);
        let mut input = String::new();
        print!("bet: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("err");
        input = input.chars().filter(|c| c.is_digit(10)).collect();

        let bet_result = input.trim().parse::<f64>();
        let bet = match bet_result {
            Ok(num) => {
                if num > wallet {
                    println!("your too broke");
                    spin_sleep::sleep(Duration::new(2, 12_550_000));
                    continue;
                }
                num
            }
            Err(_e) => {
                println!("not a num");
                println!("{}", input);
                spin_sleep::sleep(Duration::new(3, 12_550_000));
                clear_screan();
                continue;
            }
        };
        wallet -= bet;

        //deal
        deal(&mut deck, &mut player_hand, &mut dealer_hand);
        display_frame(&player_hand, &dealer_hand, &wallet);

        // player loop
        loop {
            let device_state = DeviceState::new();
            let keys: Vec<Keycode> = device_state.get_keys();

            //wait a bit
            for _i in 0..15000000 {}

            //hit
            if keys.contains(&Keycode::H) {
                hit(&mut player_hand, &mut deck);
                display_frame(&player_hand, &dealer_hand, &wallet);

                // player busts
                if count_hand(&player_hand) > 21 {
                    println!("You bust");
                    println!("You lose ${}", bet);
                    spin_sleep::sleep(Duration::new(3, 12_550_000));
                    continue 'main;
                }

                //blackjack
                if count_hand(&player_hand) == 21 {
                    spin_sleep::sleep(Duration::new(3, 12_550_000));
                    break;
                }
            }

            //stand
            if keys.contains(&Keycode::S) {
                break;
            }
        }
        dealer_hand[1].flipped_over = false;
        display_frame(&player_hand, &dealer_hand, &wallet);

        //dealer loop
        loop {
            spin_sleep::sleep(Duration::new(2, 12_550_000));
            display_frame(&player_hand, &dealer_hand, &wallet);

            let dealer_total = count_hand(&dealer_hand);
            let player_total = count_hand(&player_hand);

            if dealer_total >= 17 {
                //bust
                if dealer_total > 21 {
                    println!("Dealer busts");
                    if count_hand(&player_hand) == 21 {
                        println!("WINNER WINNER CHICKEN DINNER");
                        println!("You win ${}", bet * 1.5);
                        wallet += bet * 2.5;
                        spin_sleep::sleep(Duration::new(3, 12_550_000));
                        continue 'main;
                    }
                    println!("You win ${}", bet);
                    wallet += bet * 2.0;
                    spin_sleep::sleep(Duration::new(3, 12_550_000));
                    continue 'main;
                }

                // not bust

                //player wins
                if player_total > dealer_total {
                    //blackjack
                    if player_total == 21 {
                        println!("WINNER WINNER CHICKEN DINNER");
                        println!("You win ${}", bet * 1.5);
                        wallet += bet * 2.5;
                        spin_sleep::sleep(Duration::new(3, 12_550_000));
                        continue 'main;
                    }
                    // not blackjack but player still wins
                    println!("You win");
                    println!("You win ${}", bet);
                    wallet += bet * 2.0;
                    spin_sleep::sleep(Duration::new(3, 12_550_000));
                    continue 'main;
                }

                //dealer wins
                if dealer_total > player_total {
                    println!("You lose");
                    println!("You lose ${}", bet);
                    spin_sleep::sleep(Duration::new(3, 12_550_000));
                    continue 'main;
                }

                //tie
                if dealer_total == player_total {
                    println!("Tie");
                    wallet += bet;
                    spin_sleep::sleep(Duration::new(3, 12_550_000));
                    continue 'main;
                }
            }
            hit(&mut dealer_hand, &mut deck);
        }
    }
}

fn count_hand(hand: &Vec<Card>) -> i32 {
    let value_map = HashMap::from([
        ("2".to_string(), 2),
        ("3".to_string(), 3),
        ("4".to_string(), 4),
        ("5".to_string(), 5),
        ("6".to_string(), 6),
        ("7".to_string(), 7),
        ("8".to_string(), 8),
        ("9".to_string(), 9),
        ("10".to_string(), 10),
        ("J".to_string(), 10),
        ("Q".to_string(), 10),
        ("K".to_string(), 10),
        ("A".to_string(), 11),
    ]);

    let mut total = 0;
    for card in hand {
        total += value_map[&card.value.to_string()];
    }
    if !contains_ace(&hand) || (contains_ace(&hand) && total <= 21) {
        return total;
    }
    // hand has an ace and total is over 21
    total = 0;
    for card in hand {
        if card.value == "A".to_string() {
            total += 1;
            continue;
        }
        total += value_map[&card.value.to_string()];
    }
    return total;
}

fn contains_ace(hand: &Vec<Card>) -> bool {
    for card in hand {
        if card.value == "A" {
            return true;
        }
    }
    return false;
}

fn hit(hand: &mut Vec<Card>, deck: &mut Vec<Card>) {
    hand.push(deck[0].clone());
    deck.remove(0);
}

fn clear_screan() {
    clearscreen::clear().expect("failed to clear screen");
}

fn display_frame(player_hand: &Vec<Card>, dealer_hand: &Vec<Card>, wallet: &f64) {
    clearscreen::clear().expect("failed to clear screen");
    let mut frame: Vec<Vec<char>> = Vec::new();

    let mut row = 0;
    while row < 9 {
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
    while row < 9 {
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
    println!("={}", count_hand(&player_hand));
}

fn deal(deck: &mut Vec<Card>, player_hand: &mut Vec<Card>, dealer_hand: &mut Vec<Card>) {
    player_hand.push(deck[0].clone());
    deck.remove(0);
    player_hand.push(deck[0].clone());
    deck.remove(0);

    player_hand[0].flipped_over = false;
    player_hand[1].flipped_over = false;

    dealer_hand.push(deck[0].clone());
    deck.remove(0);
    dealer_hand.push(deck[0].clone());
    deck.remove(0);

    dealer_hand[0].flipped_over = false;
    dealer_hand[1].flipped_over = true;
}

fn create_deck() -> Vec<Card> {
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
