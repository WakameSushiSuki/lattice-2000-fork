use std::env;
use std::fs;
use std::convert::TryInto;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code: String = fs::read_to_string(args[1].clone()).expect("Unable to read file");
    let bytes = assemble(code);
    fs::write(args[2].clone(), bytes).expect("Unable to write file");
}

fn assemble(code: String) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    for line in code.lines() {
        let instruction = line.split_whitespace().collect::<Vec<&str>>();
        if instruction.len() == 0 {
            continue;
        }
        match instruction[0] {
            "ADA" => {
                bytes.push(0x01);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "ADW" => {
                bytes.push(0x02);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "ADC" => {
                bytes.push(0x03);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "SBA" => {
                bytes.push(0x04);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "SBW" => {
                bytes.push(0x05);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "SBC" => {
                bytes.push(0x06);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "MLA" => {
                bytes.push(0x07);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "MLW" => {
                bytes.push(0x08);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "MLC" => {
                bytes.push(0x09);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "DVA" => {
                bytes.push(0x0A);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "DVW" => {
                bytes.push(0x0B);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "DVC" => {
                bytes.push(0x0C);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "LMA" => {
                bytes.push(0x0D);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "LMW" => {
                bytes.push(0x0E);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "LMC" => {
                bytes.push(0x0F);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "TAW" => {
                bytes.push(0x10);
                bytes.push(0x00);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "TAC" => {
                bytes.push(0x11);
                bytes.push(0x00);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "TWA" => {
                bytes.push(0x12);
                bytes.push(0x00);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "TWC" => {
                bytes.push(0x13);
                bytes.push(0x00);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "TCA" => {
                bytes.push(0x14);
                bytes.push(0x00);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "TCW" => {
                bytes.push(0x15);
                bytes.push(0x00);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            _ => panic!("Unknown instruction"),
        }
    }
    return bytes;
}