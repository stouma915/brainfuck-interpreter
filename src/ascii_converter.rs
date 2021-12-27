use unescape::unescape;

pub fn convert_to_char(i: i16) -> Option<char> {
    let mut result = None;

    if i >= 0 && i <= 127 {
        let hex = if i < 16 {
            format!("\\x0{:x}", i)
        } else {
            format!("\\x{:x}", i)
        };

        let chars = unescape(hex.as_str())
            .unwrap()
            .chars()
            .collect::<Vec<char>>();
        result = Some(*chars.get(0).unwrap())
    }

    result
}
