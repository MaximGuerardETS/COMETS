// use clap;
use std::fs::File;
use std::io::{BufRead, BufReader, Bytes};

const CODE_VERSION:u8 = 1;

fn main() -> Result<(), Box<dyn std::error::Error>>{


    println!("COMETS database Creator!");
    let file = File::open("COMETS01.txt")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();


    let mut byte_read = reader.read_line(&mut line)?;

    assert_eq!("COMETS_DATABASE_FILE", line.trim());
    line.clear();
    byte_read = reader.read_line(&mut line)?;
    assert_eq!(format!("Version: {CODE_VERSION}"), line.trim());
    line.clear();
    byte_read = reader.read_line(&mut line)?;
    println!("{}", line.trim());
    line.clear();
    byte_read = reader.read_line(&mut line)?;
    println!("{}", line.trim());
    line.clear();
    byte_read = reader.read_line(&mut line)?;
    line.clear();
    Ok(())

}


