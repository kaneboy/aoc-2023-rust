use std::fmt::*;

// /// 立方体的类型。
// ///
// #[derive(Debug, PartialEq)]
// pub enum Cube {
//     Blue,
//     Green,
//     Red,
// }

// impl std::str::FromStr for Cube {

//     type Err = anyhow::Error;

//     fn from_str(s: &str) -> anyhow::Result<Self> {
//         match s {
//             "blue" => Ok(Cube::Blue),
//             "green" => Ok(Cube::Green),
//             "red" => Ok(Cube::Red),
//             _ => Err(anyhow::anyhow!("解析错误。")),
//         }
//     }

// }

/// 一次游戏的结果。一次游戏可以包含多个记录的结果。
///
#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub records: Vec<GameRecord>,
}

impl Display for Game {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        _ = write!(f, "Game {}: ", self.id);

        for (idx, record) in self.records.iter().enumerate() {
            if idx != self.records.len() - 1 {
                _ = write!(f, "{}; ", record);
            } else {
                _ = write!(f, "{}", record);
            }
        }

        Ok(())
    }

}

impl Game {

    pub fn is_possible(&self, total_red: u32, total_green: u32, total_blue: u32) -> bool {
        for record in &self.records {
            if !record.is_possible(total_red, total_green, total_blue) {
                return false;
            }
        }
        true
    }

}

/// 游戏中记录的一次结果，分别记录了3种颜色的立方体被拿出了多少个。
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameRecord {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Display for GameRecord {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} blue, {} red, {} green", self.blue, self.red, self.green)
    }

}

impl GameRecord {

    /// 根据给定的3种颜色立方体的总数，判断当前游戏结果是否可能存在。
    ///
    fn is_possible(&self, total_red: u32, total_green: u32, total_blue: u32) -> bool {

        self.red <= total_red &&
        self.green <= total_green &&
        self.blue <= total_blue

    }

}
