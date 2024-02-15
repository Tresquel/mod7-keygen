use std::env;
mod generator;

fn help() {
    println!("usage:");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() { 
        2 => {
            match args[1].to_lowercase().as_str() {
                "-cd" => {
                    println!("Generated CD Key: {}", generator::cd());
                    return
                }

                _ => {
                    eprintln!("Invalid argument(s)!");
                    help();
                    return
                }
            }
        }

        _ => {
            eprintln!("Invalid argument(s)!");
            help();
        }
    }

}
