//function to return ASCII value of character
fn val(c: char) -> u8 {
    match c {
        '0'..='9' => {
            c as u8 - b'0'
        }
        _ => c as u8 - b'A' + 10
    }
}

//function to convert a number
//from given base to decimal number
fn to_deci(s: String, base: u32) -> Option<u32> {
    let mut power = 1u32;
    let mut result = 0u32;

    for c in s.trim().chars().rev() {
        let v = val(c.to_ascii_uppercase()) as u32;
        if v >= base {
            return None;
        }

        result = result + v * power;
        power = power * base;
    }

    return Some(result);
}

//Function to convert a given
//decimal number to a given base
fn from_deci(base: u32, mut input: u32) -> String {
    let mut res = String::new();

    while input > 0 {
        res.push(re_val((input % base) as u8));

        input = input / base;
    }

    res.chars().rev().collect()
}

//Function to return equivalent
//character of a given value
fn re_val(num: u8) -> char {
    match num {
        0..=9 => (num + '0' as u8) as char,
        _ => (num - 10 + 'A' as u8) as char
    }
}

//Function to convert a given number
//from a base to another base
pub fn convert_base(s: String, a: u32, b: u32) -> String {
    let s = s.to_uppercase();
    if let Some(num) = to_deci(s, a) {
        return from_deci(b, num);
    }

    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_base() {
        assert_eq!(convert_base("5A".to_string(), 16, 2), "1011010".to_string());
        assert_eq!(convert_base("5a".to_string(), 16, 2), "1011010".to_string());
        assert_eq!(convert_base("1011010".to_string(), 2, 16), "5A".to_string());
    }

    #[test]
    fn test_val() {
        assert_eq!(val('0'), 0);
        assert_eq!(val('5'), 5);
        assert_eq!(val('A'), 10);
        assert_eq!(val('F'), 15);
    }

    #[test]
    fn test_to_deci() {
        assert_eq!(to_deci("F".to_string(), 16), Some(15));
        assert_eq!(to_deci("FF".to_string(), 16), Some(255));
        assert_eq!(to_deci("10101".to_string(), 2), Some(21));
        assert_eq!(to_deci("A10101".to_string(), 2), None);
    }

    #[test]
    fn test_re_val() {
        assert_eq!(re_val(0), '0');
        assert_eq!(re_val(10), 'A');
        assert_eq!(re_val(15), 'F');
    }

    #[test]
    fn test_from_deci() {
        assert_eq!(from_deci(16, 15), "F".to_string());
        assert_eq!(from_deci(16, 90), "5A".to_string());
        assert_eq!(from_deci(2, 90), "1011010".to_string());
    }
}
