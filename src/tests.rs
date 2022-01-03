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
    use crate::interpreter::eval;
    use crate::Memory;

    #[test]
    fn can_evaluate_brainfuck_code() {
        assert_eq!(
            eval(
                &String::from(
                    "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+.+."
                ),
                &mut Memory::new()
            )
            .ok()
            .unwrap()
            .content,
            "ABC"
        );
        assert_eq!(
            eval(
                &String::from("++++++++++++++++++++++++++++++++++++++++++++++++++-.+.+."),
                &mut Memory::new()
            )
            .ok()
            .unwrap()
            .content,
            "123"
        );
    }

    #[test]
    fn can_evaluate_loop_code() {
        assert_eq!(
            eval(&String::from("----[---->+<]>++.+.+."), &mut Memory::new())
                .ok()
                .unwrap()
                .content,
            "ABC"
        );
        assert_eq!(
            eval(&String::from("-[----->+<]>--.+.+."), &mut Memory::new())
                .ok()
                .unwrap()
                .content,
            "123"
        );
    }

    #[test]
    fn can_throw_an_error() {
        assert_eq!(
            eval(&String::from("[[[[["), &mut Memory::new())
                .err()
                .is_some(),
            true
        );
        assert_eq!(
            eval(&String::from("-."), &mut Memory::new())
                .err()
                .is_some(),
            true
        );
    }
}
