/// 题目：https://adventofcode.com/2023/day/6

use std::io::BufRead;

///
/// 比赛的时长和最高纪录。
///
struct RaceAndRecord(u32, u32);

fn main() {

    const FILE_PATH : &str = "./day6.txt";

    let lines : Vec<String> = match read_all_lines(FILE_PATH) {
        Ok(lines) => lines,
        Err(err) => {
            println!("读文件失败：{err}");
            return;
        },
    };

    // 所有比赛场次和它们的最高纪录。
    let race_and_records : Vec<RaceAndRecord> = match parse_all_races(&lines) {
        Ok(vec) => vec,
        Err(err) => {
            println!("解析文件失败：{err}");
            return;
        },
    };

    let mut result : Option<u32> = None;

    // 便利每个比赛，计算可以赢得当前记录的策略个数，将它们相乘。
    for race_and_record in &race_and_records {

        let winning_count : u32 = get_all_possible_race_distances(race_and_record.0)
            .iter()
            .filter(|&&s| s > race_and_record.1)
            .count() as u32;

        if winning_count > 0 {
            result = match result {
                None => Some(winning_count),
                Some(count) => Some(count * winning_count),
            };
        }
    }

    if let Some(result) = result {
        println!("结果是：{result}。");
    } else {
        println!("没有任何可以胜过的策略。");
    }
}

///
/// 从数据文件解析出所有比赛。
///
fn parse_all_races(lines: &[String]) -> anyhow::Result<Vec<RaceAndRecord>> {

    if lines.len() != 2 {
        return Err(anyhow::anyhow!("文本文件格式不正确，行数不是2。"));
    }

    let race_durations : Vec<u32> = {
        let parts : Vec<&str> = lines[0].split(':').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("文本文件格式不正确。"));
        }
        let numbers : Vec<u32> = parts[1]
            .split(' ')
            .filter_map(|s| s.parse().ok())
            .collect();
        numbers
    };

    let race_records : Vec<u32> = {
        let parts : Vec<&str> = lines[1].split(':').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("文本文件格式不正确。"));
        }
        let numbers : Vec<u32> = parts[1]
            .split(' ')
            .filter_map(|s| s.parse().ok())
            .collect();
        numbers
    };

    if race_durations.len() != race_records.len() {
        return Err(anyhow::anyhow!("文本文件格式不正确。"));
    }

    let race_and_records : Vec<RaceAndRecord> = (0..race_durations.len())
        .into_iter()
        .map(|idx| RaceAndRecord(race_durations[idx], race_records[idx]))
        .collect();

    Ok(race_and_records)
}

///
/// 根据指定比赛时长，计算所有可能的比赛成绩。
///
fn get_all_possible_race_distances(race_duration: u32) -> Vec<u32> {

    // 所有可能的按住按钮的时间，从 0ms 到 比赛全程按住。
    let press_durations = 0..=race_duration;

    press_durations
        .into_iter()
        .map(|press_duration| (race_duration - press_duration) * press_duration)
        .collect()

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
