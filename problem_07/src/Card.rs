use std::collections::HashMap;

pub const CARD_SET:&str = "23456789TJQKA";
pub enum CardResult {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
    NoCard
}
pub struct CardHand {
    id: String,
    bid: i64
}

impl Default for CardHand {
    fn default() -> Self {
        Self { id: String::default(), bid: 0 }
    }
}

impl CardHand {
    pub fn create(id: &str, bid: i64) -> Self {
        Self { id: String::from(id), bid }
    }
    pub fn get_id(self: &Self) -> &str {
        &self.id
    }
    pub fn get_bid(self: &Self) -> i64 {
        self.bid
    }
    pub fn get_power(self: &Self) -> i64  {
        match self.get_card_hand_result() {
            CardResult::HighCard => 0,
            CardResult::OnePair => 16,
            CardResult::TwoPair => 32,
            CardResult::ThreeOfAKind => 48,
            CardResult::FullHouse => 64,
            CardResult::FourOfAKind => 80,
            CardResult::FiveOfAKind => 96,
            CardResult::NoCard => i64::MIN,
        }
    }
    pub fn get_hand_power(self: &Self) -> i64  {
        let mut result:i64 = 0;
        let mut index = 0;
        for bit in [48, 40, 32, 24, 16] {
            let curr_char = self.id.chars().nth(index).unwrap();

            let position = CARD_SET.chars().position(|p| p == curr_char).unwrap();
            result |= (position as i64) << bit;
            index += 1;
        }


        return result;
    }

    pub fn get_card_hand_result(self: &Self) -> CardResult {
        let mut map:HashMap<char, i64> = HashMap::new();
        for ch in CARD_SET.chars() {
            map.insert(ch, 0);
        }

        for ch in self.id.chars() {
            if let Some(entry) = map.get_mut(&ch) {
                *entry += 1;
            }
        }

        let mut count = map
            .iter()
            .filter(|(&k,&v)| v > 0)
            .count();

        let max =  map
            .iter()
            .filter(|(&k,&v)| v > 0)
            .map(|(k, v)| *v)
            .max()
            .unwrap();

        return match count {
            5 => CardResult::HighCard,
            4 => CardResult::OnePair,
            3 => match max {
                3 => CardResult::ThreeOfAKind,
                2 => CardResult::TwoPair,
                _=> CardResult::NoCard
            },
            2 => match max {
                3 => CardResult::FullHouse,
                4 => CardResult::FourOfAKind,
                _=> CardResult::NoCard
            },
            1 => CardResult::FiveOfAKind,
            _=> CardResult::NoCard
        };

    }
}
