use std::collections::HashMap;

#[derive(Debug, PartialEq)]
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

pub fn score(dice: Dice, category: Category) -> u8 {
    match category {
        Category::Ones => dice.iter().filter(|&d| *d == 1).sum::<u8>(),
        Category::Twos => dice.iter().filter(|&d| *d == 2).sum::<u8>(),
        Category::Threes => dice.iter().filter(|&d| *d == 3).sum::<u8>(),
        Category::Fours => dice.iter().filter(|&d| *d == 4).sum::<u8>(),
        Category::Fives => dice.iter().filter(|&d| *d == 5).sum::<u8>(),
        Category::Sixes => dice.iter().filter(|&d| *d == 6).sum::<u8>(),
        Category::FullHouse | Category::FourOfAKind | Category::Yacht => {
            let mut counts: HashMap<u8, u8> = HashMap::new();

            for dice_draw in dice {
                *counts.entry(dice_draw).or_insert(0) += 1;
            }

            let mut occurences: Vec<u8> = counts.values().copied().collect();
            occurences.sort();

            if occurences == [2, 3] && category == Category::FullHouse {
                dice.iter().sum()
            } else if (occurences == [1, 4] || occurences == [5])
                && category == Category::FourOfAKind
            {
                let key_four_time = counts
                    .iter()
                    .find(|&(_, &v)| v == *occurences.iter().max().unwrap())
                    .map(|(&k, _)| k)
                    .unwrap();

                key_four_time * 4
            } else if occurences == [5] && category == Category::Yacht {
                50
            } else {
                0
            }
        }
        Category::LittleStraight | Category::BigStraight => {
            let mut dice = dice;
            dice.sort();

            if (dice == [1, 2, 3, 4, 5] && category == Category::LittleStraight)
                || (dice == [2, 3, 4, 5, 6] && category == Category::BigStraight)
            {
                30
            } else {
                0
            }
        }
        Category::Choice => dice.iter().sum::<u8>(),
    }
}
