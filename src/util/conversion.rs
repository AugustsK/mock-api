fn un_pretty(input: &str) -> Option<(u64, String)> {
    let mut number_part = String::new();
    let mut suffix_part = String::new();

    for ch in input.chars() {
        if ch.is_digit(10) {
            number_part.push(ch);
        } else {
            suffix_part.push_str(&input[number_part.len()..]);
            break;
        }
    }

    if let Ok(number) = number_part.parse::<u64>() {
        Some((number, suffix_part))
    } else {
        None
    }
}

pub fn un_pretty_bytes(input: &str) -> Option<u64> {
    if let Some((number, suffix)) = un_pretty(&input) {
        return Some(number * suffix_to_bytes(&suffix))
    } else {
        None
    }
}

fn suffix_to_bytes(input: &str) -> u64 {
    let mut bi_bytes = false;
    let mut magnitude = 0;

    for (index, ch) in input.chars().enumerate() {
        if index == 0 && ch.to_string().to_uppercase() == "K".to_string() {
            magnitude = 1;
        }

        if index == 0 && ch.to_string().to_uppercase() == "M".to_string() {
            magnitude = 2;
        }

        if index == 1 && ch.to_string().to_lowercase() == "i".to_string() {
            bi_bytes = true;
        }

        if index > 1 {
            break;
        }
    }

    let result = if bi_bytes {
        1024u64.pow(magnitude)
    } else {
        1000u64.pow(magnitude)
    };

    return result;
}

pub fn un_pretty_time(input: &str) -> Option<u64> {
    if let Some((number, suffix)) = un_pretty(&input) {
        return Some(number * suffix_to_milliseconds(&suffix))
    } else {
        None
    }
}

fn suffix_to_milliseconds(input: &str) -> u64 {
    let result = if input.to_lowercase() == "s".to_string() {
        1000
    } else if input.to_lowercase() == "m".to_string() {
        1000 * 1000
    } else {
        0
    };

    return result
}