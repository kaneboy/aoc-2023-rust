mod map;

use std::io::BufRead;
use crate::map::Map;
use crate::map::MapRule;

fn main() {

    const FILE_PATH : &str = "day5.txt";

    let lines : Vec<String> = match read_all_lines(FILE_PATH) {
        Ok(lines) => lines,
        Err(err) => {
            println!("读文件失败：{err}");
            return;
        },
    };

    let (seeds, maps) = parse_input_file(&lines).unwrap();

    println!("seeds: {}", join_to_string(" ", seeds.iter().map(|seed| seed.to_string())));
    println!();

    for map in &maps {
        println!("{map}");
    }

    let locations : Vec<u32> = seeds
        .iter()
        .filter_map(|seed| find_seed_destination_for(&maps, "location", *seed))
        .collect();

    println!("location: {}", join_to_string(" ", locations.iter().map(|location| location.to_string())));

    let lowest_location = locations.iter().min().unwrap();
    print!("最低位置：{lowest_location}");
}

fn find_seed_destination_for(maps: &[Map], destination: &str, seed: u32) -> Option<u32> {

    let mut result : u32 = seed;
    let mut next_source : &str = "seed";

    loop  {
        if let Some(map) = find_map_by_source(maps, next_source) {
            next_source = &map.destination;
            result = map.find_destination(result);
        } else {
            return None;
        }
        if next_source == destination {
            break;
        }
    }

    Some(result)
}

/// 根据 source 找到对照表。
///
fn find_map_by_source<'a>(maps: &'a [Map], source: &str) -> Option<&'a Map> {
    maps.iter()
        .find(|map| map.source == source)
}

/// 解析整个文本文件，返回 seed 集合和所有的对照表。
///
fn parse_input_file(lines: &[String]) -> anyhow::Result<(Vec<u32>, Vec<Map>)> {

    let first_line : &String = lines.first().unwrap();
    let seeds : Vec<u32> = parse_seeds_line(first_line)?;

    let mut maps : Vec<Map> = vec![];
    let mut building_map: Option<Map> = None;

    for line in lines.iter().skip(1) {
        if line.is_empty() {
            if let Some(map) = building_map {
                maps.push(map);
            }
            building_map = None;
        }
        else if building_map.is_none() {
            let (source, destination) = parse_map_source_destination_from_title(line)?;
            let map = Map {
                source,
                destination,
                rules: vec![],
            };
            building_map = Some(map);
        }
        else if building_map.is_some() {
            let map = building_map.as_mut().unwrap();
            let map_rule = parse_map_rule(line)?;
            map.rules.push(map_rule);
        }
    }

    if let Some(map) = building_map {
        maps.push(map);
    }

    Ok((seeds, maps))
}

/// 解析对照表的首行，返回 source 和 destination 。
///
fn parse_map_source_destination_from_title(line: &str) -> anyhow::Result<(String, String)> {

    // fertilizer-to-water map:

    let parts : Vec<&str> = line.split(' ').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("对照表的首行格式不正确。"));
    }

    let source_dest_parts : Vec<&str> = parts[0].split("-to-").collect();
    if source_dest_parts.len() != 2 {
        return Err(anyhow::anyhow!("对照表的首行格式不正确。"));
    }

    let source : String = source_dest_parts[0].to_owned();
    let destination : String = source_dest_parts[1].to_owned();

    Ok((source, destination))
}

/// 解析 seed 集合。
///
fn parse_seeds_line(line: &str) -> anyhow::Result<Vec<u32>> {

    // seeds: 79 14 55 13

    let parts : Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("seeds行格式不正确。"))
    }

    let seeds_str : &str = parts[1];
    let seeds_parts : Vec<&str> = seeds_str.split(' ').collect();

    let seeds : Vec<u32> = seeds_parts
        .iter()
        .filter_map(|part| part.parse::<u32>().ok())
        .collect();

    Ok(seeds)
}

/// 将一行文本解析成一个 MapRule 对象。
///
fn parse_map_rule(line: &str) -> anyhow::Result<MapRule> {

    // 50 98 2

    let parts : Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 3 {
        return Err(anyhow::anyhow!("文本行格式不正确。"));
    }

    let num1 : u32 = parts[0].parse()?;
    let num2 : u32 = parts[1].parse()?;
    let num3 : u32 = parts[2].parse()?;

    Ok(MapRule {
        destination_range_start: num1,
        source_range_start: num2,
        range_length: num3,
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

fn join_to_string(separator: &str, strings: impl IntoIterator<Item=String>) -> String {

    let mut s = String::new();

    for item in strings.into_iter() {
        s.push_str(&item);
        s.push_str(separator);
    }

    s
}
