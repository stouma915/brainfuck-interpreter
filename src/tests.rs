#[cfg(test)]
mod ascii_converter_spec {
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

#[cfg(test)]
mod interpreter_spec {
    use std::fs;

    use phf::{phf_map, Map};

    use crate::interpreter::eval;
    use crate::Memory;

    #[test]
    fn can_evaluate_brainfuck_code() {
        let test_files: Map<&str, &str> = phf_map! {
            "334.bf" => "33-4",
            "hello.bf" => "Hello.",
            "hydrogen_sound.bf" => "Ahh~! The sound of hydrogen!!"
        };

        for file in test_files.keys() {
            let expected = test_files.get(file).unwrap();

            match fs::read_to_string(format!("tests/{}", file)) {
                Ok(content) => {
                    let eval_result = eval(&content, &mut Memory::new());
                    assert_eq!(eval_result.is_ok(), true);
                    assert_eq!(eval_result.ok().unwrap().content, String::from(*expected));
                }
                Err(err) => {
                    panic!("Unable to read test file: {} ({:?})", file, err.kind());
                }
            }
        }
    }
}