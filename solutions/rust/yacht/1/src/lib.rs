#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];
use std::collections::HashSet;

use Category::*;
pub fn score(dice: Dice, category: Category) -> u8 {
    match category {
        Ones => {
            dice.into_iter().filter(|&x| x == 1).sum()
        }
        Twos => {
            dice.into_iter().filter(|&x| x == 2).sum()
        }
        Threes => {
            dice.into_iter().filter(|&x| x == 3).sum()
        }
        Fours => {
            dice.into_iter().filter(|&x| x == 4).sum()
        }
        Fives => {
            dice.into_iter().filter(|&x| x == 5).sum()
        }
        Sixes => {
            dice.into_iter().filter(|&x| x == 6).sum()
        }
        FullHouse => {
            let v = HashSet::from(dice);
            let n = dice.iter().filter(|&x| *x == dice[0]).count();
            if v.len() == 2 && (n == 2 || n ==3){
                dice.iter().sum()
            } else {
                0
            }
        }
        FourOfAKind => {
            let v = HashSet::from(dice);
            if v.len() == 1 {
                dice[0] * 4
            } else if v.len() == 2 {
                let n = dice.iter().filter(|&x| *x == dice[0]).count();
                if n == 1 {
                    dice[1] * 4
                } else if n == 2 || n == 3 {
                    0
                } else {
                    dice[0] * 4
                }
            } else {
                0
            }
        }
        LittleStraight => {
            let mut bingo = true;
            for i in 1..=5 {
                if !dice.contains(&i) {
                    bingo = false;
                }
            }
            if bingo {
                30
            } else {
                0
            }
        }
        BigStraight => {
            let mut bingo = true;
            for i in 2..=6 {
                if !dice.contains(&i) {
                    bingo = false;
                }
            }
            if bingo {
                30
            } else {
                0
            }
        }
        Choice => {
            dice.iter().sum()
        }
        Yacht => {
            let n = dice.iter().filter(|&x| *x == dice[0]).count();
            if n == 5 {
                50
            } else {
                0
            }
        }
    }
}

