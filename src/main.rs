use std::fs::File;
use std::io::{Error, Write};
use std::usize;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        let mut compiler = "compiler";
        if args.len() >= 1 {
            compiler = &args[0];
        }
        println!("Usage - {} <input file> <output file>", compiler);
        return;
    }

    match compile(&args[1], &args[2]) {
        Ok(_) => println!("Successfully compiled!"),
        Err(e) => println!("Error: {}", e),
    }
}

fn compile(input_file: &str, output_file: &str) -> Result<(), Error> {
    let contents = std::fs::read_to_string(input_file)?;
    write_to_file(convert(&contents), output_file)?;
    Ok(())
}

fn convert(s: &str) -> Vec<u8> {
    let mut bytestring = String::from(s);
    bytestring.retain(|c| c == '0' || c == '1');
    let bytes_length: usize = bytestring.len() / 8;
    let remainder = bytestring.len() % 8;
    let mut bytes: Vec<u8> = vec![];
    for i in 0..bytes_length {
        let start = i * 8;
        let end = start + 8;

        let bits = &bytestring[start..end];
        let byte: u8 = match u8::from_str_radix(bits, 2) {
            Ok(n) => n,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        bytes.push(byte);
    }

    if remainder > 0 {
        let mut padding: String = String::new();
        for _ in 0..(8 - remainder) {
            padding += "0";
        }

        let start = bytes_length;
        let end = bytes_length + remainder;
        let bits = padding + &bytestring[start..end];
        let byte: u8 = match u8::from_str_radix(&bits, 2) {
            Ok(n) => n,
            Err(e) => {
                println!("Error: {}", e);
                return bytes;
            }
        };
        bytes.push(byte);
    }

    bytes
}

fn write_to_file(data: Vec<u8>, output_path: &str) -> Result<(), Error> {
    let mut file = File::create(output_path)?;
    file.write_all(&data)?;
    Ok(())
}
