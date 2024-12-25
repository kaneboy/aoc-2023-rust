/// 题目：https://adventofcode.com/2023/day/7

mod card;
mod hand;

use std::io::BufRead;

fn main() {



}

///
/// 读取数据文件的所有行。
///
fn read_all_lines(file_path: impl AsRef<std::path::Path>) -> anyhow::Result<Vec<String>> {

    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);

    let lines : Vec<String> = reader
        .lines()
        .map_while(|l| l.ok())
        .collect();

    Ok(lines)
}
