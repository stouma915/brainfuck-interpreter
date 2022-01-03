use once_cell::sync::Lazy;
use unescape::unescape;

#[cfg(test)]
mod tests {
    use crate::ascii_converter::convert_to_char;

    #[test]
    fn convert_to_ascii_correctly() {
        assert_eq!(convert_to_char(65), Some(&'A'));
        assert_eq!(convert_to_char(66), Some(&'B'));
        assert_eq!(convert_to_char(67), Some(&'C'));
        assert_eq!(convert_to_char(68), Some(&'D'));

        assert_eq!(convert_to_char(128), None);
        assert_eq!(convert_to_char(129), None);
        assert_eq!(convert_to_char(130), None);
        assert_eq!(convert_to_char(131), None);
    }
}

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
