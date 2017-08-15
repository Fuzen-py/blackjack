extern crate rand;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use rand::Rng;

fn get_deck() -> HashMap<String, u32> {
    let mut cards = HashMap::new();
    cards.insert("ACE".to_owned(), 11);
    cards.insert("TWOS".to_owned(), 2);
    cards.insert("THREES".to_owned(), 3);
    cards.insert("FOURS".to_owned(), 4);
    cards.insert("FIVES".to_owned(), 5);
    cards.insert("SIXES".to_owned(), 6);
    cards.insert("SEVENS".to_owned(), 7);
    cards.insert("EIGHTS".to_owned(), 8);
    cards.insert("NINES".to_owned(), 9);
    cards.insert("TENS".to_owned(), 10);
    cards.insert("JACKS".to_owned(), 10);
    cards.insert("KINGS".to_owned(), 10);
    cards
}

fn draw_card(value: u32) -> u32 {
    let deck = get_deck();
    let keys = deck.keys().collect::<Vec<&String>>();
    let nv = deck[keys[rand::thread_rng().gen_range(0, keys.len())]];
    if (nv == 11) & (value > 10) {
        value + 1
    } else {
        nv + value
    }
}

fn cmp_hit(value: u32) -> u32 {
    if value >= 20 {
        return value;
    };
    if rand::thread_rng().gen_range(0, 10) > 7 {
        return value;
    }
    return draw_card(value);
}

fn main() {
    //let input =  io::stdin().lock().read_line();
    //let bet = input.parse::<u64>().unwrap()
    let mut player1 = draw_card(0);
    let mut player2 = draw_card(0);
    loop {
        println!("You: {}", player1);
        println!("Robot: {}", player2);
        println!("HIT / STAY");
        let mut txt = String::new();
        let input = io::stdin();
        let _ = input.lock().read_line(&mut txt).is_ok();
        txt = txt.to_lowercase().trim().to_owned();
        if txt == "hit" {
            player1 = draw_card(player1);
            player2 = cmp_hit(player2);
            if (player1 >= 21) | (player2 >= 21) {
                break;
            }
        } else if txt == "stay" {
            if player2 > player1 {
                break;
            }
            player2 = cmp_hit(player2);
            break;
        }
    }
    if player1 == player2 {
        println!("It's a tie!");
        return;
    }
    if player1 == 21 {
        println!("You Win!");
        return;
    }
    if player2 == 21 {
        println!("You Win!");
        return;
    }
    if player1 > 21 {
        println!("You lose!");
        return;
    }
    if player2 > 21 {
        println!("You lose!");
        return;
    }
    if player1 > player2 {
        println!("You Win!");
        return;
    }
    println!("You lose!");
}
