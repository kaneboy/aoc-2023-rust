use crate::pos::Pos;

/// 引擎示意图里面的符号。
///
pub struct Symbol {
    /// 符号在引擎图上的定位点。一个符号只占用一个位置。
    pub position: Pos,
}
