use std::{cmp::Ordering, collections::HashMap};
use crate::card::Card;

///
/// 一手牌，一共五张牌。
///
#[derive(PartialEq)]
pub struct Hand {
    cards: [Card; 5],
}

impl Hand {

    pub fn new(cards: [Card; 5]) -> Hand {
        Hand { cards }
    }

    fn get_rank(&self) -> usize {

        // 把所有牌放进一个字典，key是牌面，value是此牌面出现的次数。
        let card_map = self.put_into_map();

        // 字典里面只有一个entry，说明5张牌都相同。
        if card_map.len() == 1 {
            return 6;
        }

        // 字典有两个entry：
        //   - 其中有一个entry的值是4：有4张牌相同。
        //   - 否则是 full house。
        if card_map.len() == 2 {
            if card_map.iter().any(|bucket| *bucket.1 == 4) {
                return 5;
            } else {
                return 4;
            }
        }

        // 有3张牌相同。
        if card_map.iter().any(|bucket| *bucket.1 == 3) {
            return 3;
        }

        // 两个对子。
        if card_map.len() == 3 &&
           card_map.iter().filter(|entry| *entry.1 == 2).count() == 2 {
            return 2;
        }

        // 一个对子。
        if card_map.iter().filter(|entry| *entry.1 == 2).count() == 1 {
            return 1;
        }

        // 杂牌。
        0
    }

    ///
    /// 将一手牌的所有牌放进一个字典：key 是牌面，value 是这个牌面有多少张。
    ///
    fn put_into_map(&self) -> HashMap<Card, usize> {

        let mut map : HashMap<Card, usize> = HashMap::new();

        for card in &self.cards {
            if let Some(num) = map.get_mut(card) {
                *num += 1;
            } else {
                map.insert(*card, 1);
            }
        }

        map
    }

}

//
// 为 Hand 实现大小比较，比较的逻辑基于牌面。
//
impl PartialOrd<Hand> for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {

        let self_rank = self.get_rank();
        let other_rank = other.get_rank();

        if self_rank > other_rank {
            return Some(Ordering::Greater);
        }
        if self_rank < other_rank {
            return Some(Ordering::Less);
        }

        for i in 0..5 {
            match self.cards[i].cmp(&other.cards[i]) {
                Ordering::Greater => return Some(Ordering::Greater),
                Ordering::Less    => return Some(Ordering::Less),
                _                 => continue,
            }
        }

        Some(Ordering::Equal)
    }
}
