pub mod alphabet_translator {
    use std::num::ParseIntError;

    const VALID_CHARS: [char; 61] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'y', 'z', 'A',
        'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
        'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    /// Takes in a hex code an return the associated `char`
    pub fn hex_to_char(hex: &str) -> Result<char, ParseIntError> {
        let numeric_code = u8::from_str_radix(&hex, 16);
        match numeric_code {
            Err(e) => Err(e),
            Ok(code) => Ok(code as char),
        }
    }

    /// Takes in a `char` and returns xHH where HH is the hex code. It will add
    /// a `0` to the front if the code would normally only be one character
    pub fn char_to_hex(c: char) -> String {
        let mut temp = String::new();
        let n = c as u8;
        let mut hex_code = format!("{:X}", n);
        if hex_code.len() == 1 {
            hex_code.insert(0, '0');
        }
        temp.push('x');
        temp.push_str(&hex_code);
        temp
    }

    /// Alphabet encodes a string using `char_to_hex`
    pub fn char_to_hex_a_string(input: &str) -> String {
        input.chars().map(|c| char_to_hex(c)).collect()
    }

    /// will convert a char to alphabet encoded only if required to by the
    /// specification
    pub fn char_to_hex_if_required(c: char) -> String {
        if !VALID_CHARS.contains(&c) {
            return char_to_hex(c);
        }
        c.to_string()
    }

    /// will convert only the characters that need to be converted within a
    /// string
    pub fn char_to_hex_str_if_required(input: &str) -> String {
        input.chars().map(|c| char_to_hex_if_required(c)).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::alphabet_translator::*;

    #[test]
    fn char_to_hex_if_required_tests() {
        assert_eq!("a", char_to_hex_if_required('a'));
        assert_eq!("x20", char_to_hex_if_required(' '));
        assert_eq!("x0A", char_to_hex_if_required('\n'));
        assert_eq!("x5C", char_to_hex_if_required('\\'));
        assert_eq!("k", char_to_hex_if_required('k'));
    }

    #[test]
    #[should_panic]
    fn bad_hex_to_char() {
        hex_to_char("AY").unwrap();
    }

    #[test]
    fn trans_with_wacks() {
        assert_eq!("x61x62x5Cx61x62", char_to_hex_a_string("ab\\ab"));
    }
    #[test]
    fn simple_str_trans() {
        assert_eq!("x61x62", char_to_hex_a_string("ab"));
    }

    #[test]
    fn char_to_hex_tests() {
        assert_eq!("x61", char_to_hex('a'));
        assert_eq!("x20", char_to_hex(' '));
        assert_eq!("x0A", char_to_hex('\n'));
        assert_eq!("x5C", char_to_hex('\\'));
    }
    #[test]
    fn basic_hex_to_char_hex() {
        assert_eq!('a', hex_to_char("61").unwrap());
        assert_eq!(' ', hex_to_char("20").unwrap());
        assert_eq!('\n', hex_to_char("0A").unwrap());
        assert_eq!('\\', hex_to_char("5C").unwrap());
    }
}
