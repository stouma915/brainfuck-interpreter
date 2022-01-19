use once_cell::sync::Lazy;
use unescape::unescape;

static LOOKUP_TABLE: Lazy<Vec<String>> = Lazy::new(|| {
    (0..127)
        .map(|n| format!("\\x{:02x}", n))
        .map(|n| unescape(n.as_str()).unwrap())
        .map(|n| n.chars().collect::<Vec<char>>())
        .map(|n| *n.first().unwrap())
        .map(|n| n.to_string())
        .collect()
});

pub fn convert(i: i16) -> Option<&'static str> {
    LOOKUP_TABLE.get(i as usize).map(|value| &*value.as_str())
}
