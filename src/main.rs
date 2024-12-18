use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Получаем аргументы командной строки
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Использование: {} <файл> <текст>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let search_text = &args[2];

    // Открываем файл
    let file = File::open(filename).map_err(|e| {
        eprintln!("Ошибка при открытии файла: {}", e);
        e
    })?;

    // Создаем регулярное выражение
    let regex = Regex::new(search_text)?;

    // Читаем файл построчно
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if regex.is_match(&line) {
            println!("{}", line);
        }
    }

    Ok(())
}

