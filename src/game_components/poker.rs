use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
enum Suit {
    Spade(u8),
    Heart(u8),
    Club(u8),
    Diamond(u8),
    Joker(u8),
}

impl Suit {
    fn default_vec(has_joker: bool) -> Vec<Suit> {
        let mut list = Vec::with_capacity(5);
        list.push(Suit::Spade(4));
        list.push(Suit::Heart(3));
        list.push(Suit::Club(2));
        list.push(Suit::Diamond(1));
        if has_joker {
            list.push(Suit::Joker(5));
        }
        list
    }
}

impl Suit {
    fn as_str(&self) -> &'static str {
        match self {
            Suit::Spade(_) => "♠️",
            Suit::Heart(_) => "♥️",
            Suit::Club(_) => "♣️",
            Suit::Diamond(_) => "♦️",
            Suit::Joker(_) => "🃏",
        }
    }

    fn as_value(&self) -> u8 {
        match *self {
            Suit::Spade(val) => val,
            Suit::Heart(val) => val,
            Suit::Club(val) => val,
            Suit::Diamond(val) => val,
            Suit::Joker(val) => val,
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone)]
struct Poker {
    suit: Suit,
    text: String,
    point: u32,
}

impl Poker {
    fn new(suit: Suit, text: String, point: u32) -> Poker {
        Poker {
            suit,
            text,
            point,
        }
    }
}

impl Display for Poker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.suit.as_str(), self.text)
    }
}

#[derive(Debug)]
struct PokerBox {
    pokers: Vec<Poker>
}

impl PokerBox {
    fn new(has_joker: bool) -> PokerBox {
        let suits = Suit::default_vec(false);
        let mut pokers = Vec::with_capacity(54);
        for i in suits {
            for j in 1..=13 {
                match j {
                    x @ 1..=10 => {
                        pokers.push(Poker::new(i, x.to_string(), 0));
                    }
                    11 => pokers.push(Poker::new(i, String::from("J"), 0)),
                    12 => pokers.push(Poker::new(i, String::from("Q"), 0)),
                    13 => pokers.push(Poker::new(i, String::from("K"), 0)),
                    _ => println!("111")
                }
            }
        }
        if has_joker {
            pokers.push(Poker::new(Suit::Joker(0), String::from("Red Joker"), 0));
            pokers.push(Poker::new(Suit::Joker(0), String::from("Black Joker"), 0));
        }
        PokerBox { pokers }
    }
}

impl PokerBox {
    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.pokers.shuffle(&mut rng)
    }
}

use std::fmt::Write as FmtWrite;
use rand::prelude::SliceRandom;

impl Display for PokerBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buff = String::new();
        for (index, item) in self.pokers.iter().enumerate() {
            if index > 0 && index % 13 == 0 {
                write!(&mut buff, "\n")?;
            }
            write!(&mut buff, "[ {}, {} ]", item.suit.as_str(), item.text)?;
        }
        write!(f, "{}", buff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_components::poker::Suit::*;
    use rand::Rng;

    #[test]
    fn test() {
        let mut pokers = PokerBox::new(false);
        println!("{}\n", pokers);
        pokers.shuffle();
        println!("{}", pokers);
    }
}
