use clap::{Arg, Command};
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::num::ParseIntError;

#[inline]
fn main() {
    let matches = Command::new("dec2hex")
        .version("1.0")
        .author("Toa Toastrejn <toa@toaaa.de>")
        .about("Converts a decimal number to hexadecimal")
        .arg(Arg::new("DECIMAL")
            .help("The decimal number to convert")
            .required(false)
            .index(1))
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .value_name("FILE")
            .help("Input file containing decimal numbers"))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("OUTPUT")
            .help("Output file to write hexadecimal numbers"))
        .get_matches();

    if let Some(file_path) = matches.get_one::<String>("file") {
        let output_path = matches.get_one::<String>("output");
        if let Err(e) = process_file(file_path, output_path) {
            eprintln!("Error processing file: {}", e);
        }
    } else if let Some(decimal_str) = matches.get_one::<String>("DECIMAL") {
        match dec_to_hex(decimal_str) {
            Ok(hex) => {
                if let Some(output_path) = matches.get_one::<String>("output") {
                    if let Err(e) = write_to_file(output_path, &hex) {
                        eprintln!("Error writing to file: {}", e);
                    }
                } else {
                    println!("{}", hex);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        eprintln!("Please provide a decimal number or a file to process.");
    }
}

fn process_file(file_path: &str, output_path: Option<&String>) -> io::Result<()> {
    let input_file = File::open(file_path)?;
    let reader = BufReader::new(input_file);

    let decimals: Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .collect();

    let hex_values: Vec<String> = decimals.par_iter()
        .filter_map(|line| dec_to_hex(line).ok())
        .collect();

    if let Some(output_path) = output_path {
        let mut output_file = File::create(output_path)?;
        for hex in hex_values {
            writeln!(output_file, "{}", hex)?;
        }
    } else {
        for hex in hex_values {
            println!("{}", hex);
        }
    }

    Ok(())
}

fn write_to_file(output_path: &str, hex: &str) -> io::Result<()> {
    let mut output_file = File::create(output_path)?;
    writeln!(output_file, "{}", hex)?;
    Ok(())
}

#[inline]
fn dec_to_hex(decimal_str: &str) -> Result<String, ParseIntError> {
    decimal_str.parse::<u32>().map(|decimal_number| {
        let mut buffer = [0u8; 8];
        let mut i = buffer.len();

        let mut number = decimal_number;
        while number > 0 {
            i -= 1;
            buffer[i] = match number % 16 {
                0..=9 => b'0' + (number % 16) as u8,
                10..=15 => b'A' + (number % 16 - 10) as u8,
                _ => unreachable!(),
            };
            number /= 16;
        }

        let hex_str = std::str::from_utf8(&buffer[i..]).unwrap();
        hex_str.to_string()
    })
}