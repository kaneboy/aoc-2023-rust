/// 题目：https://adventofcode.com/2023/day/1

use std::{io::*, path::Path};

fn main() {

    let file_path = "./data.txt";

    let lines : Vec<String> = match read_all_lines(file_path) {
        Err(err) => {
            println!("读文件'{file_path}'失败：{err}");
            return;
        },
        Ok(lines) => lines,
    };

    let numbers : Vec<u32> = lines
        .iter()
        .map(|line| assembly_line_number(line))
        .collect();

    let total : u32 = numbers.iter().sum();

    println!("结果是：{total}。")
}

/// 解析一行文本，使用第1个和最后1个单位数字，组成一个两位数。
///
///     "pqr3stu8vwx" -> 38
///     "treb7uchet" -> 77
///     "abc" -> 0
///
fn assembly_line_number(line: &str) -> u32 {

    let digits : Vec<u32> = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    match digits.len() {
        0 => 0u32,
        _ => digits[0] * 10 + digits[digits.len() - 1],
    }
}

/// 读取数据文件的所有行。
///
fn read_all_lines(file_path: impl AsRef<Path>) -> anyhow::Result<Vec<String>> {

    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);

    let lines : Vec<String> = reader
        .lines()
        .map_while(|l| l.ok())
        .collect();

    anyhow::Ok(lines)
}
