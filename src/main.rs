use std::fs;
use std::path::Path;
use rand::Rng;
use std::process;
use std::env;

const QUOTE_MIN: usize = 150;
const QUOTE_MAX: usize = 400;
const USAGE: &str ="
Available commands:
-h                            This screen right here.
-o <short,medium,long>        Show short,medium or long quotes only.
-c <red, blue, green, etc>    Add some color. Use after -o command.
";

#[derive(Debug)]
enum QuoteSize {
    Short,
    Medium,
    Long,
    Default,
}

#[derive(Debug)]
enum QuoteColor {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    None,
}

#[derive(Debug)]
struct Quote<'a> {
    quote: String,
    data: Vec<&'a str>,
    size: QuoteSize,
    color: QuoteColor,
}

impl<'a> Quote<'a> {
    fn init(data: Vec<&'a str>) -> Quote<'a> {
        Quote { 
            quote: "None".to_string(), 
            data: data, 
            size: QuoteSize::Default,
            color: QuoteColor::None
        }
    }

    fn size(&mut self) {
        let mut tmp = vec![];

        match &self.size {
            QuoteSize::Short => {
                for q in &self.data {
                    if q.len() <= QUOTE_MIN {
                        tmp.push(q);
                    }
                }
            }
            QuoteSize::Medium => {
                for q in &self.data {
                    if q.len() > QUOTE_MIN && q.len() < QUOTE_MAX {
                        tmp.push(q)
                    }
                }
            }
            QuoteSize::Long => {
                for q in &self.data {
                    if q.len() >= QUOTE_MAX {
                        tmp.push(q);
                    }
                }
            }
            _ => {
                for q in &self.data {
                    tmp.push(q)
                }
            }
        }

        let mut r_thread = rand::thread_rng();
        let random = r_thread.gen_range(0, tmp.len() - 1);
        self.quote = tmp[random].to_string();
    }

    fn color(&mut self) -> Option<String> {
        let end = "\x1b[0m";
        match self.color {
            QuoteColor::Red => Some("\x1B[31m".to_string() + &self.quote + end),
            QuoteColor::Green => Some("\x1B[32m".to_string() + &self.quote + end),
            QuoteColor::Yellow => Some("\x1B[33m".to_string() + &self.quote + end),
            QuoteColor::Blue => Some("\x1B[34m".to_string() + &self.quote + end),
            QuoteColor::Magenta => Some("\x1B[35m".to_string() + &self.quote + end),
            QuoteColor::Cyan => Some("\x1B[36m".to_string() + &self.quote + end),
            QuoteColor::None => None,
        }
    }

    fn get(&mut self) {
        self.size();

        match &self.color() {
            Some(v) => {
                println!("{}", v.to_string());
            },
            None => {
                println!("{}", self.quote)
            }
        };

    }
}

fn main() {
    let file = read_file().unwrap().to_owned();
    let data: Vec<&str> = file.split("\n%\n").collect();
    let mut quotes = Quote::init(data);

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
                        "short" => {
                            quotes.size = QuoteSize::Short
                            },
                        "medium" => {
                            quotes.size = QuoteSize::Medium
                            },
                        "long" => {
                            quotes.size = QuoteSize::Long
                            },
                        _ => println!("Use short, medium or long.")
                    }
                }
            }
            _ => {
                println!("No such command.");
                process::exit(1);
            }
        }
    }

    if args.len() > 3 {
        match args[3].to_lowercase().as_str() {
            "-c" | "--c" => {
                if args.len() > 4 {
                   match args[4].to_lowercase().as_str() {
                        "red" => quotes.color = QuoteColor::Red,
                        "green" => quotes.color = QuoteColor::Green,
                        "yellow" => quotes.color = QuoteColor::Yellow,
                        "blue" => quotes.color = QuoteColor::Blue,
                        "magenta" => quotes.color = QuoteColor::Magenta,
                        "cyan" => quotes.color = QuoteColor::Cyan,
                        _ => quotes.color = QuoteColor::None,
                    } 
                }
            }
            _ => {
                println!("None")
            }
        }
    }

    quotes.get();
}

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