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
        s_segment = random.gen_range(7..9999997);
        if digit_sum(s_segment) % 7 != 0 || s_segment % 10 >= 8 {
            continue;
        } else {
            break
        }
    }

    format!("{:0>3}-{:0>7}", f_segment, s_segment)
}

fn digit_sum(mut n: i32) -> i32 {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}