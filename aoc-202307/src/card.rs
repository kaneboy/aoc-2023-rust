///
/// 牌。
///
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
    N2,  // Rust不允许纯数字作为枚举值，使用 ‘N2’ 表示数字牌 '2' 。
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

//
// 实现从 char 到 Card 的转换。
//
impl TryFrom<char> for Card {

    type Error = anyhow::Error;

    fn try_from(ch : char) -> anyhow::Result<Card> {
        match ch {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'T' => Ok(Card::T),
            '9' => Ok(Card::N9),
            '8' => Ok(Card::N8),
            '7' => Ok(Card::N7),
            '6' => Ok(Card::N6),
            '5' => Ok(Card::N5),
            '4' => Ok(Card::N4),
            '3' => Ok(Card::N3),
            '2' => Ok(Card::N2),
            _ => Err(anyhow::anyhow!("Not a valid card.")),
        }
    }

}
