use regex::Regex;

pub fn check(key: String) -> String {
    /*
        CD key - 11 chars
        Office key - 12 chars
        OEM key - 23 chars
    */

    match key.len() {
        11 => {
            if check_cd(key) {
                return "CD".to_string()
            }
        }

        12 => {
            if check_office(key) {
                return "Office".to_string()
            }
        }

        23 => {
            if check_oem(key) {
                return "OEM".to_string()
            }
        }
        _ => {
            return String::new() // return an empty type to indicate that the key is invalid
        }
    }
    String::new()
}

fn check_cd(key: String) -> bool {
    // check if the key has the right pattern
    let pattern = Regex::new(r"^\d{3}-\d{7}$").unwrap();
    if !pattern.is_match(&key) { 
        return false;
    }

    let parts: Vec<&str> = key.split("-").collect();
    // check if this segment contains banned numbers
    let banned_numbers = [333, 444, 555, 666, 777, 888, 999];

    let f_segment: i32 = parts[0].trim().parse().unwrap();
    if banned_numbers.contains(&f_segment) {
        return false;
    }
    // check if this segment is divisible by 7 and if the last digit isnt => 8 or 0
    let s_segment: i32 = parts[1].trim().parse().unwrap();
    if digit_sum(s_segment) % 7 != 0 || s_segment % 10 >= 8 || s_segment % 10 == 0 {
        return false;
    }

    true
}

fn check_office(key: String) -> bool {
    true
}

fn check_oem(key: String) -> bool {
    true
}

fn digit_sum(mut n: i32) -> i32 {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}
