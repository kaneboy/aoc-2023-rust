use crate::pos::Pos;

/// 引擎原理图中的零件号。它不一定是一个有效的零件号。
///
#[derive(Debug)]
pub struct PartNumber {
    /// 零件号的值。
    pub val: u32,
    /// 零件号在引擎图上所占用的所有定位点。一个零件号可能占用多个位置。
    pub positions: Vec<Pos>,
}
