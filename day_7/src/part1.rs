use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};
#[derive(Debug, Clone)]
struct Hand {
    cards: String,
    bid: i32,
}
pub fn execute() {
    let start = std::time::Instant::now();
    let file = File::open("day_7/data1.txt").expect("Unable to open day 7 data 1");
    let reader = io::BufReader::new(file);
    // let mut hands_vec: Vec<Hand> = vec![];
    // let mut bids_vec: Vec<i32> = vec![];
    let mut hands: Vec<Hand> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let (hand, bid) = line.split_once(' ').unwrap();
        // hands_vec.push(hand.to_owned());
        // bids_vec.push(bid.parse::<i32>().unwrap());
        hands.push(Hand {
            cards: hand.to_owned(),
            bid: bid.parse::<i32>().unwrap(),
        });
        //get flush kinds,
        //compare hands in different flush kinds,
        //combine to make a single Vec for getting values
    }
    // how to save the combinations? Hashmap showing?
    // check number of times that the char appears... and record the type of flush there....
    let mut hand_map: HashMap<String, Hands> = HashMap::new();
    // Order the hands depending on their deck kind
    for hand in hands.iter() {
        let mut temp_cards: Vec<String> = vec![];
        for card in hand.cards.chars() {
            let card = card.to_string();
            if !temp_cards.contains(&card) {
                temp_cards.push(card);
            }
        }
        match temp_cards.len() {
            1 => {
                if let Some(temp_hand) = hand_map.get_mut("five_kind") {
                    temp_hand.hands.push(hand.clone());
                } else {
                    let hands = vec![hand.clone()];
                    hand_map.insert("five_kind".to_string(), Hands { hands });
                }
                // str8
            }
            2 => {
                let mut inc: i32 = 0;
                for temp_card in temp_cards.iter() {
                    inc = 0;
                    for card in hand.cards.chars() {
                        if card == *temp_card.chars().collect::<Vec<char>>().first().unwrap() {
                            inc += 1;
                        }
                    }
                    if inc > 3 {
                        break;
                    }
                }
                if inc > 3 {
                    if let Some(temp_vec) = hand_map.get_mut("four_kind") {
                        temp_vec.hands.push(hand.clone());
                    } else {
                        let hands = vec![hand.clone()];
                        hand_map.insert("four_kind".to_string(), Hands { hands });
                    }
                } else if let Some(temp_vec) = hand_map.get_mut("full_house") {
                    temp_vec.hands.push(hand.clone());
                } else {
                    let hands = vec![hand.clone()];
                    hand_map.insert("full_house".to_string(), Hands { hands });
                }

                // TODO four_kind and full house
            }
            3 => {
                let mut inc: i32 = 0;
                for temp_card in temp_cards.iter() {
                    inc = 0;
                    for card in hand.cards.chars() {
                        if card == *temp_card.chars().collect::<Vec<char>>().first().unwrap() {
                            inc += 1;
                        }
                    }
                    if inc > 2 {
                        break;
                    }
                }
                if inc > 2 {
                    if let Some(hands) = hand_map.get_mut("three_kind") {
                        hands.hands.push(hand.clone());
                    } else {
                        let hands = vec![hand.clone()];
                        hand_map.insert("three_kind".to_string(), Hands { hands });
                    }
                // TODO three_kind
                } else if let Some(hands) = hand_map.get_mut("two_pair") {
                    hands.hands.push(hand.clone());
                } else {
                    let hands = vec![hand.clone()];
                    hand_map.insert("two_pair".to_string(), Hands { hands });
                }
                // TODO two_pair
            }

            4 => {
                if let Some(hands) = hand_map.get_mut("one_pair") {
                    hands.hands.push(hand.clone());
                } else {
                    let hands = vec![hand.clone()];
                    hand_map.insert("one_pair".to_string(), Hands { hands });
                }
                // one_pair
            }
            5 => {
                if let Some(hands) = hand_map.get_mut("high_card") {
                    hands.hands.push(hand.clone());
                } else {
                    let hands = vec![hand.clone()];
                    hand_map.insert("high_card".to_string(), Hands { hands });
                }
                // three
            }
            _ => {
                panic!("OOOOPPPPSS")
            } // compare cards in similar flushes
        }
    }
    let _ = hand_map
        .iter_mut()
        .map(|(_, hands)| {
            hands.sort_category();
        })
        .collect::<()>();
    let mut rank: i32 = 1;
    let mut rank_total: i32 = 0;

    let _ = hand_map
        .get("high_card")
        .unwrap()
        .hands
        .iter()
        .map(|hand| {
            rank_total += rank * hand.bid;
            rank += 1;
        })
        .collect::<()>();
    let _ = hand_map
        .get("one_pair")
        .unwrap()
        .hands
        .iter()
        .map(|hand| {
            rank_total += rank * hand.bid;
            rank += 1;
        })
        .collect::<()>();
    let _ = hand_map
        .get("two_pair")
        .unwrap()
        .hands
        .iter()
        .map(|hand| {
            rank_total += rank * hand.bid;
            rank += 1;
        })
        .collect::<()>();
    let _ = hand_map
        .get("three_kind")
        .unwrap()
        .hands
        .iter()
        .map(|hand| {
            rank_total += rank * hand.bid;
            rank += 1;
        })
        .collect::<()>();
    let _ = hand_map
        .get("full_house")
        .unwrap()
        .hands
        .iter()
        .map(|hand| {
            rank_total += rank * hand.bid;
            rank += 1;
        })
        .collect::<()>();
    let _ = hand_map
        .get("four_kind")
        .unwrap()
        .hands
        .iter()
        .map(|hand| {
            rank_total += rank * hand.bid;
            rank += 1;
        })
        .collect::<()>();
    let _ = hand_map
        .get("five_kind")
        .unwrap()
        .hands
        .iter()
        .map(|hand| {
            rank_total += rank * hand.bid;
            rank += 1;
        })
        .collect::<()>();

    println!("Logging for rank: {rank}, and rank_total: {rank_total}");
    println!("Time elapsed: {:#?}", start.elapsed());
}
#[derive(Debug, Clone)]
struct Hands {
    hands: Vec<Hand>,
}
impl Hands {
    fn sort_category(&mut self) {
        let mut swapped;
        for i in 0..self.hands.len() {
            for j in 0..self.hands.len() - i - 1 {
                swapped = false;
                for k in 0..self.hands[j].cards.len() {
                    let curr = self.hands[j].cards.chars().nth(k).unwrap();
                    let next = self.hands[j + 1].cards.chars().nth(k).unwrap();
                    if curr.is_numeric() && next.is_numeric()
                        || curr.is_alphabetic() && next.is_numeric()
                        || curr.is_numeric() && next.is_alphabetic()
                    {
                        if curr > next {
                            (self.hands[j + 1], self.hands[j]) =
                                (self.hands[j].clone(), self.hands[j + 1].clone());
                            break;
                        } else if curr == next {
                            continue;
                        }
                    } else if curr.is_alphabetic() && next.is_alphabetic() {
                        match (curr, next) {
                            //situations that need to swap
                            // A and the rest, K and Q, the rest and T
                            //situations that should not swap
                            // the rest and A, Q and K, T and the rest
                            ('A', 'T' | 'K' | 'Q' | 'J') | ('K', 'Q') | ('K' | 'J' | 'Q', 'T') => {
                                (self.hands[j + 1], self.hands[j]) =
                                    (self.hands[j].clone(), self.hands[j + 1].clone());
                                break;
                            }
                            ('T' | 'K' | 'Q' | 'J', 'A') | ('Q', 'K') | ('T', 'K' | 'J' | 'Q') => {}
                            (_, _) => {
                                if curr == next {
                                    continue;
                                } else if curr > next {
                                    (self.hands[j + 1], self.hands[j]) =
                                        (self.hands[j].clone(), self.hands[j + 1].clone());
                                    break;
                                }
                            }
                        }
                    }
                    if !swapped {
                        break;
                    }
                }
            }
        }
    }
}
