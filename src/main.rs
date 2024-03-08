use std::env;

mod generator;
mod validator;

fn help() {
    println!(
        "usage:
--cd | -c
    Generates a CD key
--office | -e 
    Generates a Office key
--oem | -o
    Generates a OEM key
--check <string>
    Checks if the provided key is valid

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
                "--cd" | "-c" => {
                    println!("Generated CD key(s): ");
                    for _ in 0..amount {
                        println!("{}", generator::cd());
                    }

                    return;
                }

                "--office" | "-e" => {
                    println!("Generated Office key(s): ");
                    for _ in 0..amount {
                        println!("{}", generator::office());
                    }

                    return;
                }

                "--oem" | "-o" => {
                    println!("Generated OEM key(s): ");
                    for _ in 0..amount {
                        println!("{}", generator::oem());
                    }

                    return;
                }

                "--check" => {
                    if args.len() < 3 {
                        eprintln!("Not enough arguments!");
                        help();
                        return;
                    }
                    
                    let key = &args[2];
                    let key_type = validator::check(key.to_string()); 
                    if key_type != "" {
                        println!("{} is a valid {} key.", key, key_type)
                    } else {
                        println!("{} is a invalid key.", key)
                    }

                }

                "--help" | "-h" => {
                    help();
                    return;
                }

                "--version" | "-v" => {
                    println!("mod7-keygen version {}", env!("CARGO_PKG_VERSION"));
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
            return;
        }
    }
}
