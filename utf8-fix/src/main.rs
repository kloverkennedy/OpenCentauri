use std::{char::REPLACEMENT_CHARACTER, env, fs::read, path::PathBuf, process::exit};

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} file", args.get(0).unwrap_or(&String::from("utf8-fix")));
        exit(1);
    }

    let path = PathBuf::from(&args[1]);

    if !path.exists() {
        eprintln!("File not found: {}", path.display());
        exit(1);
    }

    let content = match read(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file {}: {}", path.display(), e);
            exit(1);
        }
    };

    let mut fixed = String::from_utf8_lossy(&content).to_string();
    drop(content);
    

    let replacement_character_count = fixed.chars().filter(|&c| c == REPLACEMENT_CHARACTER).count();

    if replacement_character_count > 0 {
        println!("Found {} invalid UTF-8 sequences in file {}", replacement_character_count, path.display());
    } else {
        println!("No invalid UTF-8 sequences found in file {}", path.display());
        exit(0);
    }

    fixed = fixed.replace(REPLACEMENT_CHARACTER, "_");

    match std::fs::write(&path, fixed) {
        Ok(_) => {
            println!("Fixed file {}", path.display());
            exit(0);
        }
        Err(e) => {
            eprintln!("Error writing file {}: {}", path.display(), e);
            exit(1);
        }
    }
}
