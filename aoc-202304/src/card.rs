use std::collections::HashSet;
use std::fmt::*;

/// 刮刮卡
///
#[derive(Debug)]
pub struct Card {
    pub id: u32,
    /// 中奖号码列表。
    pub winning_numbers: HashSet<u32>,
    /// 刮出来的号码。
    pub numbers_you_have: Vec<u32>,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        _ = write!(f, "Card {}: ", self.id);
        for n in &self.winning_numbers {
            _ = write!(f, "{} ", n);
        }
        _ = write!(f, "| ");
        for (idx, n) in self.numbers_you_have.iter().enumerate() {
            _ = write!(f, "{}", n);
            if idx != self.numbers_you_have.len() - 1 {
                _ = write!(f, " ");
            }
        }
        Ok(())
    }
}

impl Card {

    /// 计算当前刮刮卡的分数。
    ///
    pub fn score(&self) -> u32 {

        let mut result = 0u32;

        for n in &self.numbers_you_have {
            if !self.winning_numbers.contains(n) {
                continue;
            }
            result = match result {
                0 => 1,
                _ => result * 2,
            };
        }

        result
    }

}

