/// 题目：https://adventofcode.com/2023/day/3

mod part_number;
mod pos;
mod symbol;

use std::io::BufRead;
use crate::part_number::PartNumber;
use crate::symbol::Symbol;
use crate::pos::Pos;

fn main() {

    const FILE_PATH : &str = "./day3.txt";

    let lines = match read_all_lines(FILE_PATH) {
        Ok(lines) => lines,
        Err(err) => {
            println!("读数据文件失败{err}");
            return;
        },
    };

    let mut part_numbers : Vec<PartNumber> = vec![];
    let mut symbols : Vec<Symbol> = vec![];

    // 解析所有行，将解析得到的零件号和符号附加到2个Vec内。
    parse_data_file(&lines, &mut part_numbers, &mut symbols);

    // 过滤出有效的零件号(周围存在任何符号)。
    let valid_part_numbers = part_numbers
        .iter()
        .filter(|part_number| is_adjacent_to_any_symbol(part_number, &symbols));

    // 累加有效零件号的值。
    let part_numbers_sum : u32 = valid_part_numbers
        .map(|part_number| part_number.val)
        .sum();

    println!("所有零件号的总和是：{part_numbers_sum}");

}

/// 检查零部件是否挨着任何一个符号？
///
fn is_adjacent_to_any_symbol(part_number: &PartNumber, symbols: &[Symbol]) -> bool {

    // part_number.positions
    //     .iter()
    //     .any(|pos| symbols.iter().any(|symbol| pos.is_adjacent(&symbol.position)))

    for pos in &part_number.positions {
        for symbol in symbols {
            if pos.is_adjacent(&symbol.position) {
                return true;
            }
        }
    }

    false
}

/// 解析整个文件的全部文本行，将解析到的零件号和符号放入参数。
///
fn parse_data_file(
    lines: &[String],
    part_numbers: &mut Vec<PartNumber>,
    symbols: &mut Vec<Symbol>)
{
    for (line_index, line) in lines.iter().enumerate() {

        let mut number = Some(0u32);
        let mut number_positions : Vec<Pos> = vec![];

        for (char_index, char) in line.chars().enumerate() {

            let pos = Pos {
                x: line_index as u32,
                y: char_index as u32,
            };

            if char.is_ascii_digit() {
                let digit : u32 = char.to_digit(10).unwrap();
                number = match number {
                    Some(val) => Some(val * 10 + digit),
                    None => Some(digit),
                };
                number_positions.push(pos);
                continue;
            }

            if char != '.' {
                symbols.push(Symbol { position: pos });
            }

            if let Some(num) = number {
                let part_number = PartNumber {
                    val: num,
                    positions: number_positions,
                };
                part_numbers.push(part_number);
                number = None;
                number_positions = vec![];
            }

        }
    }
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

