use std::fmt::*;

/// 对照表。
///
pub struct Map {
    pub source: String,
    pub destination: String,
    pub rules: Vec<MapRule>,
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        _ = writeln!(f, "{}-to-{} map:", self.source, self.destination);
        for rule in &self.rules {
            _ = writeln!(f, "{}", rule);
        }
        Ok(())
    }
}

impl Map {

    /// 根据输入的 source ，从对照表中找到对应的 destination 。找不到则返回 None 。
    ///
    pub fn find_destination(&self, source: u32) -> u32 {
        for rule in &self.rules {
            if let Some(destination) = rule.find_destination(source) {
                return destination;
            }
        }
        source
    }

}

/// 对照表上的一条规则。
///
pub struct MapRule {
    pub destination_range_start: u32,
    pub source_range_start: u32,
    pub range_length: u32,
}

impl Display for MapRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.destination_range_start, self.source_range_start, self.range_length)
    }
}

impl MapRule {

    /// 根据输入的 source ，从对照表中找到对应的 destination 。找不到则返回 None 。
    ///
    fn find_destination(&self, source: u32) -> Option<u32> {

        let source_range_end = self.source_range_start + self.range_length;

        if source >= self.source_range_start && source < source_range_end {
            let offset = source - self.source_range_start;
            return Some(self.destination_range_start + offset);
        }

        None
    }

}
