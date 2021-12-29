use lazy_static::lazy_static;
use unescape::unescape;

lazy_static! {
    static ref LOOKUP_TABLE: Vec<char> = (0..127)
        .map(|n| format!("\\x{:02x}", n))
        .map(|n| unescape(n.as_str()).unwrap())
        .map(|n| n.chars().collect::<Vec<char>>())
        .map(|n| *n.get(0).unwrap())
        .collect();
}

pub fn convert_to_char(i: i16) -> Option<char> {
    LOOKUP_TABLE.get(i as usize).map(|e| *e)
}
