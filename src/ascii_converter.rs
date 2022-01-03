use once_cell::sync::Lazy;
use unescape::unescape;

static LOOKUP_TABLE: Lazy<Vec<char>> = Lazy::new(|| {
    (0..127)
        .map(|n| format!("\\x{:02x}", n))
        .map(|n| unescape(n.as_str()).unwrap())
        .map(|n| n.chars().collect::<Vec<char>>())
        .map(|n| *n.get(0).unwrap())
        .collect()
});

pub fn convert_to_char(i: i16) -> Option<&'static char> {
    LOOKUP_TABLE.get(i as usize)
}
