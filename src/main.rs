use std::fs;
use std::io;
use std::path::Path;
use rand::Rng;
use std::process;
use std::env;

const QUOTE_MIN: usize = 150;
const QUOTE_MAX: usize = 400;
const USAGE: &str ="
Available commands:
  -h                        This screen right here.
  -o <short,medium,long>    Show short,medium or long quotes only.
";

enum QuoteSize {
    Short,
    Medium,
    Long,
    Default,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].to_lowercase().as_str() {
            "-h" | "--h" => {
                println!("{}", USAGE);
                process::exit(1);
            }
            "-o" | "--o" => {
                if args.len() > 2 {
                    match args[2].to_lowercase().as_str() {
                        "short" => get_quote(QuoteSize::Short),
                        "medium" => get_quote(QuoteSize::Medium),
                        "long" => get_quote(QuoteSize::Long),
                        _ => println!("Use short, medium or long.")
                    }
                }
            }
            "-m" | "--m" => {
                unimplemented!();
            }
            _ => println!("No such command.")
        }
    } 
    else {
        get_quote(QuoteSize::Default);
    }
    
    Ok(())
}

fn get_quote(arg: QuoteSize) {
    let file = read_file().unwrap();
    let quotes: Vec<&str> = file.split("\n%\n").collect();
    let mut tmp = vec![];

    match arg {
        QuoteSize::Short => {
            for q in &quotes {
                if q.len() <= QUOTE_MIN {
                    tmp.push(q)
                }
            }
        }
        QuoteSize::Medium => {
            for q in &quotes {
                if q.len() > QUOTE_MIN && q.len() < QUOTE_MAX {
                    tmp.push(q)
                }
            }
        }
        QuoteSize::Long => {
            for q in &quotes {
                if q.len() >= QUOTE_MAX {
                    tmp.push(q)
                }
            }
        }
        _ => {
            for q in &quotes {
                tmp.push(q)
            }
        }
    }
    println!("{}", tmp[random(tmp.len())])
}

fn random(i: usize) -> usize {
    let mut r_thread = rand::thread_rng();
    r_thread.gen_range(0, i)
}

// TODO: Implement Colors.
// fn color(x: String) -> String {
//     let start = "\x1B[31m".to_string();
//     let end = "\x1B[0m".to_string();
//     start + &x + &end
// }

fn read_file() -> Result<String, &'static str> {
    let quotebase = match directory("fortunes") {
        Ok(n) => n,
        Err(err) => {
            eprintln!("Error finding file: {}", err);
            process::exit(1);
        }
    };

    let file = match fs::read_to_string(quotebase) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            process::exit(1);
        }
    };

    Ok(file)
}

fn directory<F: AsRef<Path>>(file: F) -> Result<std::path::PathBuf, &'static str> {
    let exe_path = match std::env::current_exe() {
        Ok(f) => f,
        Err(_) => return Err("Could not find executable."),
    };
    
    let exe_parent_path = match exe_path.parent() {
        Some(f) => f,
        None => return Err("Can't get executable's parent path."),
    };

    let path = exe_parent_path.join(file);

    if path.exists() { 
        Ok(path) 
    } else {
        Err("Path not found.")
    }
}