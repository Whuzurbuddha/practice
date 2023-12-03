use std::fs;

pub fn main() {
    let mut _values: Vec<String> = Vec::new();
    let input_file = "src/three_values/input";
    if let Ok(file_content) = fs::read_to_string(input_file) {
        _values = file_content.lines().map(String::from).collect();
    }
    let mut _numbers_vector = Vec::new();
    for _i in 0.._values.len() {
        let mut chars = _values[_i].chars().peekable();
        while let Some(_j) = chars.next() {
            if _j.is_ascii_punctuation() {
                let mut _number = String::new();
                let mut _start = _j;
                while let Some(_next_char) = chars.peek() {
                    if _next_char.is_digit(10) {
                        _number.push(*_next_char);
                        chars.next();
                    } else {
                        let _end = *_next_char;
                        if _start == '.' && _end == '.' {
                            _number.clear();
                        }
                        break;
                    }
                }
                if !_number.is_empty() {
                    _numbers_vector.push(_number.clone());
                }
            }
        }
    }
    println!("{:?}", _numbers_vector);
}
