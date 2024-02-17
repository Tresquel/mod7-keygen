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

pub fn check_cd(key: String) -> bool {
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

pub fn check_office(key: String) -> bool {
    // check if the key has the right pattern
    let pattern = Regex::new(r"^\d{4}-\d{7}$").unwrap();
    if !pattern.is_match(&key) { 
        return false;
    }

    let parts: Vec<&str> = key.split("-").collect();
    // check if the first segment is correct
    let f_segment: i32 = parts[0].trim().parse().unwrap();

    let third_digit = (f_segment / 10) % 10;
    let last_digit = f_segment % 10;
    if third_digit == 9 && last_digit != 0 && last_digit != 1{
        return false;
    } else if last_digit != third_digit + 1 {
        return false;
    }

    // check if this segment is divisible by 7 and if the last digit isnt => 8 or 0
    let s_segment: i32 = parts[1].trim().parse().unwrap();
    if digit_sum(s_segment) % 7 != 0 || s_segment % 10 >= 8 || s_segment % 10 == 0 {
        return false;
    }

    true
}

pub fn check_oem(key: String) -> bool {
    let pattern = Regex::new(r"^\d{5}-OEM-\d{7}-\d{5}$").unwrap();
    if !pattern.is_match(&key) { 
        return false;
    }

    let parts: Vec<&str> = key.split("-").collect();
    // check the day
    let day = &parts[0][..3].parse().unwrap();
    let days = 1..366;
    if !days.contains(day) {
        return false;
    }
    // check the year
    let valid_years = [95, 96, 97, 98, 99, 00, 01, 02]; // 03 is techically a valid year but W95 doesn't accept it so I'm leaving it out
    let year: &i32 = &parts[0][3..5].parse().unwrap();
    if !valid_years.contains(year) {
        return false;
    }
    // check if the first digit is a 0
    let s_segment: i32 = parts[2].trim().parse().unwrap();
    if parts[2].chars().next().unwrap() != '0' {
        return false;
    }

    if digit_sum(s_segment) % 7 != 0 || s_segment % 10 >= 8 || s_segment % 10 == 0 {
        return false;
    }

    // we don't need to check the last part because its already the right length
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


#[cfg(test)]
mod tests {
    use crate::validator::{check_cd, check_oem, check_office};

    #[test]
    fn test_cd_key() {
        assert!(check_cd("111-1111111".to_string()))
    }

    #[test]
    fn test_office_key() {
        assert!(check_office("1112-1111111".to_string()))
    }

    #[test]
    fn test_oem_key() {
        assert!(check_oem("00100-OEM-0000007-00000".to_string()))
    }
}