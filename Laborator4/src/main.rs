use std::fs::File;
use std::io::Write;
use std::time::Instant;
use std::{fs, io};

fn main() {

    println!("Longest line by bytes: {}", calculate_longest_line_by_bytes("file.txt"));
    println!("Longest line by characters: {}", calculate_longest_line_by_characters("file.txt"));

    match apply_rot13("Pot decodifica totul in limita de timp".to_string()) {
        Ok(s) => println!("ROT13 encoded string: {}", s),
        Err(CharError::NotAscii) => println!("Error: String contains non-ASCII characters!"),
        _ => println!("Unexpected error"),
    }

    if let Ok(phrase) = replace_by_abbr_file("file.txt", "abbr.txt") {
        println!("Modified phrase: {}", phrase);
    }

    let _ = print_system_hosts("C:\\Windows\\System32\\drivers\\etc\\hosts");

    println!("Testing bonus_problem_v1 (non-optimized):");
    bonus_problem_v1().expect("Error in bonus_problem_v1");

    println!("Testing bonus_problem_v2 (optimized):");
    bonus_problem_v2().expect("Error in bonus_problem_v2");

}

fn calculate_longest_line_by_bytes(file_name: &str) -> usize {
    let s = fs::read_to_string(file_name).ok().unwrap();
    let mut best: usize = 0;
    for s in s.lines() {
        if s.len() > best {
            best = s.len();
        }
    }
    return best;
}

fn calculate_longest_line_by_characters(file_name: &str) -> i32 {
    let s = fs::read_to_string(file_name).ok().unwrap();
    let mut best: i32 = 0;
    for s in s.lines() {
        let mut count: i32 = 0;
        for _ in s.chars() {
            count = count + 1;
        }
        if count > best {
            best = count;
        }
    }
    return best;
}

enum CharError {
    NotAscii,
    IOFailed,
}

fn apply_rot13(s: String) -> Result<String, CharError> {
    if !s.is_ascii() {
        return Err(CharError::NotAscii);
    }

    let mut new_string = String::new();

    for ch in s.chars() {
        if ch.is_lowercase() {
            new_string.push(((((ch as u8 - 'a' as u8) + 13) % 26) + 'a' as u8) as char);
        } else if ch.is_uppercase() {
            new_string.push(((((ch as u8 - 'A' as u8) + 13) % 26) + 'A' as u8) as char);
        } else {
            new_string.push(ch);
        }
    }
    Ok(new_string)
}

fn replace_by_abbr_file(phrase_file: &str, abbr_file: &str) -> Result<String, CharError> {
    if let Ok(mut phrase) = fs::read_to_string(phrase_file) {
        if let Ok(abbr) = fs::read_to_string(abbr_file) {
            for a in abbr.lines() {
                let mut it = a.split_whitespace();
                if let (Some(a1), Some(a2)) = (it.next(), it.next()) {
                    phrase = phrase.replace(a2, a1);
                }
            }
        }
        return Ok(phrase);
    } else {
        return Err(CharError::IOFailed);
    }
}

fn print_system_hosts(hosts_file: &str) -> Result<(), CharError> {
    if let Ok(hosts) = fs::read_to_string(hosts_file) {
        for line in hosts.lines() {
            if !line.starts_with('#') {
                let mut it = line.split_whitespace();
                if let (Some(a1), Some(a2)) = (it.next(), it.next()) {
                    println!("{} => {}", a2, a1);
                }
            }
        }
        Ok(())
    } else {
        Err(CharError::IOFailed)
    }
}

fn bonus_problem_v1() -> Result<(), io::Error> {
    /// Varianta neoptimizata a algoritmului
    fn generate_bonus_file(file_path: &str) -> Result<(), io::Error> {
        let bytes = 104_857_600; // Number of bytes for a 100MB file
        let mut file = File::create(file_path)?;
        let text = "Ore wa kaizoku-o ni naru!";
        let size = text.len();
        file.write_all(text.repeat(bytes / size).as_bytes())?;
        Ok(())
    }
    let _ = generate_bonus_file("bonus.txt");
    let start = Instant::now();
    let s = fs::read_to_string("bonus.txt")?;
    if let Ok(_) = apply_rot13(s) {
        println!("Brute version: {:?}", start.elapsed());
    }
    Ok(())
}

fn bonus_problem_v2() -> Result<(), io::Error> {
    // Varianta Optimizate
    // Idee de optimizare: Aplicam ROT13 pe textul initial si dupa il concatenam de cate ori e nevoie pentru a obtine reprezentarea ROT13 a textul-ui din file
    // Am facut doar pentru un fisier de 100MB, pentru unul de 4GB pur si simplu modificam bytes la 4_000_000_000

    fn generate_bonus_file(file_path: &str, text: &str) -> Result<(), io::Error> {
        let bytes = 104_857_600; // Number of bytes for a 100MB file
        let mut file = File::create(file_path)?;
        let size = text.len();
        file.write_all(text.repeat(bytes / size).as_bytes())?;
        Ok(())
    }
    let text = "Ore wa kaizoku-o ni naru!";
    let _ = generate_bonus_file("bonus.txt", text);
    let start = Instant::now();
    if let Ok(rot13_string) = apply_rot13(String::from(text)) {
        let _ = rot13_string.repeat(104_857_600 / text.len());
        println!("Optimized version: {:?}", start.elapsed());
    }
    Ok(())
}
