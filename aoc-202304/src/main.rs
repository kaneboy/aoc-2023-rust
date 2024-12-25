/// 题目：https://adventofcode.com/2023/day/4

mod card;

use std::collections::HashSet;
use std::io::BufRead;
use crate::card::Card;

fn main() {

    const FILE_PATH : &str = "./day4.txt";

    let lines = match read_all_lines(FILE_PATH) {
        Ok(lines) => lines,
        Err(err) => {
            println!("读文件失败：{err}");
            return;
        },
    };

    let cards = lines
        .iter()
        .filter_map(|line| parse_card(line).ok())
        .inspect(|card| println!("{card}"));

    let total_score : u32 = cards
        .map(|card| card.score())
        .sum();

    println!("总分数是：{total_score}");
}

/// 将一行文本解析成 Card 对象。
///
fn parse_card(line: &str) -> anyhow::Result<Card> {

    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53

    let parts : Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("文本行格式不正确。"));
    }

    let card_id_parts : Vec<&str> = parts[0].split(' ').collect();
    if card_id_parts.len() != 2 {
        return Err(anyhow::anyhow!("文本行格式不正确。"));
    }

    let card_id : u32 = match card_id_parts[1].parse() {
        Ok(id) => id,
        Err(_) => return Err(anyhow::anyhow!("文本行格式不正确。")),
    };

    let numbers_parts : Vec<&str> = parts[1].split('|').collect();
    if numbers_parts.len() != 2 {
        return Err(anyhow::anyhow!("文本行格式不正确。"));
    }

    let mut winning_numbers = HashSet::<u32>::new();
    let mut numbers_you_have = Vec::<u32>::new();

    for number_str in numbers_parts[0].split(' ') {
        if let Ok(num) = number_str.parse() {
            winning_numbers.insert(num);
        }
    }

    for number_str in numbers_parts[1].split(' ') {
        if let Ok(num) = number_str.parse() {
            numbers_you_have.push(num);
        }
    }

    Ok(Card {
        id: card_id,
        winning_numbers,
        numbers_you_have,
    })
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
