extern crate rand;

use rand::Rng;

pub fn cd() -> String {
    let banned_numbers = [333, 444, 555, 666, 777, 888, 999];
    let mut random = rand::thread_rng();

    let mut f_segment: i32; // 111

    // loop forever until we find a valid first segment number thingy
    loop {
        f_segment = random.gen_range(0..999);
        if banned_numbers.contains(&f_segment) {
            continue; // start over
        } else {
            break;
        }
    }

    let mut s_segment: i32; // 1111111

    // loop forever until we find a valid second segment thingy
    loop {
        s_segment = random.gen_range(7..=9999997);
        if digit_sum(s_segment) % 7 != 0 || s_segment % 10 >= 8 || s_segment % 10 == 0 {
            continue;
        } else {
            break;
        }
    }

    format!("{:0>3}-{:0>7}", f_segment, s_segment)
}

pub fn office() -> String {
    let mut random = rand::thread_rng();

    let mut f_segment = random.gen_range(1..=999); // 1112
    let mut last_digit = f_segment % 10;
    if last_digit == 9 {
        // 9 + 1 = 10 so just wrap it down to 0
        last_digit = 0;
    } else {
        last_digit += 1;
    }
    f_segment = (f_segment.to_string() + &last_digit.to_string())
        .parse()
        .unwrap(); // combine the 3 digits with the other one

    let mut s_segment: i32; // 1111111

    // loop forever until we find a valid second segment thingy
    loop {
        s_segment = random.gen_range(7..=9999997);
        if digit_sum(s_segment) % 7 != 0 || s_segment % 10 >= 8 || s_segment % 10 == 0 {
            continue;
        } else {
            break;
        }
    }

    format!("{:0>4}-{:0>7}", f_segment, s_segment)
}

pub fn oem() -> String {
    let mut random = rand::thread_rng();

    let valid_years = [95, 96, 97, 98, 99, 00, 01, 02];
    let day = random.gen_range(1..=366);
    let mut year = valid_years[random.gen_range(0..valid_years.len())]; // year 03 is invalid for W95

    let f_segment = format!("{:0>3}{:0>2}", day, year);

    let mut s_segment: i32; // 1111111

    // loop forever until we find a valid second segment thingy
    loop {
        s_segment = random.gen_range(7..=999997);
        if digit_sum(s_segment) % 7 != 0 || s_segment % 10 >= 8 || s_segment % 10 == 0 {
            continue;
        } else {
            break;
        }
    }

    let t_segment = random.gen_range(0..=99999);

    format!("{}-OEM-{:0>7}-{:0>5}", f_segment, s_segment, t_segment)
}

fn digit_sum(mut n: i32) -> i32 {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}
