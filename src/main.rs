// update this file with your own tests
use std::fs::OpenOptions;
use std::env;
use std::io;
use std::io::{BufRead, BufReader, Write, Seek, SeekFrom};

fn main() {
    let args : Vec<String> = env::args().collect();
    println!("{:?}", args);
    let file = OpenOptions::new().read(true).write(true).create(true).open(&args[1]);
    match file {
        Ok(mut file) => {
            let mut message = String::new();
            println!("Ecrivez dans le fichier : ");
            io::stdin().read_line(&mut message).expect("Errur d'entree");
            writeln!(file, "{}", message).expect("Erreur d'ecriture");
            file.seek(SeekFrom::Start(0)).unwrap();
            let reader = BufReader::new(file);
            for line in reader.lines() {
                match line {
                    Ok(line) => println!("{}", line),
                    Err(error) => {
                        panic!("Error reading line: {}", error)
                    }
                }
            }
        }
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
    };
}
