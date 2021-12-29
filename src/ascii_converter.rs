use lazy_static::lazy_static;
use unescape::unescape;

lazy_static! {
    static ref LOOKUP_TABLE: Vec<String> = (0..127).map(|n| format!("\\x{:02x}", n)).collect();
}

pub fn convert_to_char(i: i16) -> Option<char> {
    if i >= 0 && i <= 127 {
        let escaped = LOOKUP_TABLE.get(i as usize).unwrap().to_owned();

        let chars = unescape(escaped.as_str())
            .unwrap()
            .chars()
            .collect::<Vec<char>>();

        Some(*chars.get(0).unwrap())
    } else {
        None
    }
}
