const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn encode_base64(input: &[u8]) -> String {
    let mut output = String::new();
    let mut i = 0;
    
    while i < input.len() {
        let b1 = input[i];
        let b2 = if i + 1 < input.len() { input[i + 1] } else { 0 };
        let b3 = if i + 2 < input.len() { input[i + 2] } else { 0 };
        
        let triple = ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32);
        
        output.push(BASE64_CHARS[((triple >> 18) & 0x3F) as usize] as char);
        output.push(BASE64_CHARS[((triple >> 12) & 0x3F) as usize] as char);
        
        if i + 1 < input.len() {
            output.push(BASE64_CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            output.push('=');
        }
        
        if i + 2 < input.len() {
            output.push(BASE64_CHARS[(triple & 0x3F) as usize] as char);
        } else {
            output.push('=');
        }
        
        i += 3;
    }
    
    output
}

fn decode_base64(input: &str) -> Option<Vec<u8>> {
    let input = input.trim_end_matches('=');
    if input.len() % 4 == 1 {
        return None;
    }
    
    let mut output = Vec::new();
    let mut i = 0;
    
    while i < input.len() {
        let b1 = decode_char(input.chars().nth(i)?)?;
        let b2 = decode_char(input.chars().nth(i + 1)?)?;
        let b3 = if i + 2 < input.len() { decode_char(input.chars().nth(i + 2)?)? } else { 0 };
        let b4 = if i + 3 < input.len() { decode_char(input.chars().nth(i + 3)?)? } else { 0 };
        
        let triple = (b1 << 18) | (b2 << 12) | (b3 << 6) | b4;
        
        output.push(((triple >> 16) & 0xFF) as u8);
        if i + 2 < input.len() {
            output.push(((triple >> 8) & 0xFF) as u8);
        }
        if i + 3 < input.len() {
            output.push((triple & 0xFF) as u8);
        }
        
        i += 4;
    }
    
    Some(output)
}

fn decode_char(c: char) -> Option<u32> {
    match c {
        'A'..='Z' => Some((c as u8 - b'A') as u32),
        'a'..='z' => Some((c as u8 - b'a' + 26) as u32),
        '0'..='9' => Some((c as u8 - b'0' + 52) as u32),
        '+' => Some(62),
        '/' => Some(63),
        _ => None,
    }
}

use clap::{Parser, Subcommand};
use std::io::{self, Read, Write};

#[derive(Parser)]
#[command(name = "base64")]
#[command(about = "A base64 encoder/decoder CLI tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {

    Encode {
        text: Option<String>,
    },
    Decode {
        text: Option<String>,
    },
}

fn read_from_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode { text } => {
            let input = match text {
                Some(t) => t,
                None => read_from_stdin().expect("Failed to read from stdin"),
            };
            let encoded = encode_base64(input.as_bytes());
            println!("{}", encoded);
        }
        Commands::Decode { text } => {
            let input = match text {
                Some(t) => t,
                None => read_from_stdin().expect("Failed to read from stdin"),
            };
            match decode_base64(&input) {
                Some(decoded) => {
                    match String::from_utf8(decoded) {
                        Ok(text) => println!("{}", text),
                        Err(_) => {
                            io::stderr().write_all(b"Error: Invalid UTF-8 in decoded data\n").unwrap();
                            std::process::exit(1);
                        }
                    }
                }
                None => {
                    io::stderr().write_all(b"Error: Invalid base64 input\n").unwrap();
                    std::process::exit(1);
                }
            }
        }
    }
}
