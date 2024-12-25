/// 引擎原理图中的一个定位点。
///
///     x和y表示行数和列数，都从0开始编号。从上往下、从左往右递增。
///     {x: 0, y: 0} 表示第0行第0列。
///
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Pos {
    pub x: u32,
    pub y: u32,
}

impl Pos {

    /// 判断两个定位点是否相邻？
    ///
    pub fn is_adjacent(&self, other: &Pos) -> bool {
        let dx = self.x as i64 - other.x as i64;
        let dy = self.y as i64 - other.y as i64;
        (dx.abs() <= 1) && (dy.abs() <= 1)
    }

}
