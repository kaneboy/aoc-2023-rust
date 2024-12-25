/// 题目：https://adventofcode.com/2023/day/2
///
/// 如果袋子中仅有 12 个红色立方体、13 个绿色立方体和 14 个蓝色立方体，则可以进行哪些游戏？游戏 ID 的 sum 值？

mod game;

use std::io::BufRead;
use crate::game::{Game, GameRecord};

fn main() {

    const FILE_PATH : &str = "./day2.txt";
    const TOTAL_RED : u32 = 12;
    const TOTAL_GREEN : u32 = 13;
    const TOTAL_BLUE : u32 = 14;

    let lines : Vec<String> = match read_all_lines(FILE_PATH) {
        Ok(lines) => lines,
        Err(err) => {
            println!("读取数据文件失败：{err}");
            return;
        },
    };

    let result : u32 = lines
        .iter()
        .filter_map(|line| parse_game(line).ok())   // 将每行文本解析成 Game 对象。
        .filter(|game| game.is_possible(TOTAL_RED, TOTAL_GREEN, TOTAL_BLUE)) // 过滤出可行的 Game 。
        .inspect(|game| println!("可行：{game}")) // 打印可行的 Game 。
        .map(|game| game.id) // 返回可行 Game 的 ID 。
        .sum(); // 计算 ID 总和。

    println!("可行Game的ID累计值是：{result}。");

}

/// 从数据文件的一行文本，解析出一场游戏。
///
fn parse_game(line: &str) -> anyhow::Result<Game> {

    // 每行的格式：
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

    // 使用 ':' 分割整行。
    let line_parts : Vec<&str> = line.split(':').collect();

    if line_parts.len() != 2 {
        return Err(anyhow::anyhow!("文本行格式错误，没有包含1个冒号(:)。"));
    }

    // 解析 "Game 1" ，解析出编号。
    let game_id : u32 = {
        let game_id_parts : Vec<&str> = line_parts[0].split(' ').collect();
        if game_id_parts.len() != 2 {
            return Err(anyhow::anyhow!("文本行格式错误，Game编号头部解析失败。"));
        }
        match game_id_parts[1].parse::<u32>() {
            Ok(id) => id,
            Err(_) => return Err(anyhow::anyhow!("文本行格式错误，Game编号头部解析失败。")),
        }
    };

    let mut records = vec![];

    // 使用 ';' 分割 "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green" 。
    for record_str in line_parts[1].split(';') {

        let mut record = GameRecord {
            blue: 0,
            green: 0,
            red: 0,
        };

        // 使用 ',' 分割 "3 blue, 4 red" 。
        for cube_str in record_str.split(',') {
            let cube_parts : Vec<&str> = cube_str.trim().split(' ').collect();
            if cube_parts.len() != 2 {
                continue;
            }
            match cube_parts[1] {
                "blue" => record.blue += cube_parts[0].parse::<u32>().unwrap_or_default(),
                "green" => record.green += cube_parts[0].parse::<u32>().unwrap_or_default(),
                "red" => record.red += cube_parts[0].parse::<u32>().unwrap_or_default(),
                _ => {},
            }
        }

        records.push(record);
    }

    let game = Game {
        id: game_id,
        records,
    };

    Ok(game)
}

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
