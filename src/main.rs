use std::env;
mod generator;

fn help() {
    println!(
        "usage:
-cd
    Generates a CD key
-office 
    Generates a Office key
-oem
    Generates a OEM key

optional arguments:
amount <integer>
    For example: `-cd 15` will generate 15 CD keys
"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 | 3 => {
            let mut amount: i32 = 1;
            if args.len() >= 3 {
                amount = args[2].parse().unwrap_or(1);
            }

            match args[1].to_lowercase().as_str() {
                "-cd" => {
                    println!("Generated CD key(s): ");
                    for _ in 0..amount {
                        println!("{}", generator::cd());
                    }

                    return;
                }

                "-office" => {
                    println!("Generated Office key(s): ");
                    for _ in 0..amount {
                        println!("{}", generator::office());
                    }

                    return;
                }

                "-oem" => {
                    println!("Generated OEM key(s): ");
                    for _ in 0..amount {
                        println!("{}", generator::oem());
                    }

                    return;
                }

                "-help" => {
                    help();
                    return;
                }
                _ => {
                    eprintln!("Invalid argument(s)!");
                    help();
                    return;
                }
            }
        }

        _ => {
            eprintln!("Invalid argument(s)!");
            help();
        }
    }
}
