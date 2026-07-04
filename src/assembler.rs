use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code: String = fs::read_to_string(args[1].clone()).expect("Unable to read file");
    let bytes = assemble(code);
    fs::write(args[2].clone(), bytes).expect("Unable to write file");
}

fn assemble(code: String) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut labels: HashMap<String, u8> = HashMap::new();
    let mut offset: u8 = 0;
    for line in code.lines() {
        let instruction = line.split_whitespace().collect::<Vec<&str>>();
        if instruction.len() == 0 {
            continue;
        }
        match instruction[0] {
            ".label" => {
                let label = instruction[1].to_string();
                labels.insert(label, offset); // (AOI * 2)
                if offset == 0 {
                    offset = 0;
                }
                else {
                    offset -= 2;
                }
            }
            "NOP" => {
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "ADA" => {
                bytes.push(0x01);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "ADW" => {
                bytes.push(0x02);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "ADC" => {
                bytes.push(0x03);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "SBA" => {
                bytes.push(0x04);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "SBW" => {
                bytes.push(0x05);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "SBC" => {
                bytes.push(0x06);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "MLA" => {
                bytes.push(0x07);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "MLW" => {
                bytes.push(0x08);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "MLC" => {
                bytes.push(0x09);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "DVA" => {
                bytes.push(0x0A);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "DVW" => {
                bytes.push(0x0B);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "DVC" => {
                bytes.push(0x0C);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "LMA" => {
                bytes.push(0x0D);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "LMW" => {
                bytes.push(0x0E);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "LMC" => {
                bytes.push(0x0F);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "TAW" => {
                bytes.push(0x10);
                bytes.push(0x00);
            }
            "TAC" => {
                bytes.push(0x11);
                bytes.push(0x00);
            }
            "TWA" => {
                bytes.push(0x12);
                bytes.push(0x00);
            }
            "TWC" => {
                bytes.push(0x13);
                bytes.push(0x00);
            }
            "TCA" => {
                bytes.push(0x14);
                bytes.push(0x00);
            }
            "TCW" => {
                bytes.push(0x15);
                bytes.push(0x00);
            }
            "ICA" => {
                bytes.push(0x16);
                bytes.push(0x00);
            }
            "ICW" => {
                bytes.push(0x17);
                bytes.push(0x00);
            }
            "ICC" => {
                bytes.push(0x18);
                bytes.push(0x00);
            }
            "DCA" => {
                bytes.push(0x19);
                bytes.push(0x00);
            }
            "DCW" => {
                bytes.push(0x1A);
                bytes.push(0x00);
            }
            "DCC" => {
                bytes.push(0x1B);
                bytes.push(0x00);
            }
            "CPA" => {
                bytes.push(0x1C);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "CPW" => {
                bytes.push(0x1D);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "CPC" => {
                bytes.push(0x1E);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "ORA" => {
                bytes.push(0x1F);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "ORW" => {
                bytes.push(0x20);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "ORC" => {
                bytes.push(0x21);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "ANA" => {
                bytes.push(0x22);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "ANW" => {
                bytes.push(0x23);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "ANC" => {
                bytes.push(0x24);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "XRA" => {
                bytes.push(0x25);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "XRW" => {
                bytes.push(0x26);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "XRC" => {
                bytes.push(0x27);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "NTA" => {
                bytes.push(0x28);
                bytes.push(0x00);
            }
            "NTW" => {
                bytes.push(0x29);
                bytes.push(0x00);
            }
            "NTC" => {
                bytes.push(0x2A);
                bytes.push(0x00);
            }
            "HLT" => {
                bytes.push(0xFF);
                bytes.push(0x00);
            }
            "LDA" => {
                bytes.push(0x2B);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "LDW" => {
                bytes.push(0x2C);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "LDC" => {
                bytes.push(0x2D);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "STA" => {
                bytes.push(0x2E);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "STW" => {
                bytes.push(0x2F);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "STC" => {
                bytes.push(0x30);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "LAI" => {
                bytes.push(0x31);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "LWI" => {
                bytes.push(0x32);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "LCI" => {
                bytes.push(0x33);
                let addr = instruction[1].parse::<u8>().unwrap();
                bytes.push(addr);
            }
            "PHA" => {
                bytes.push(0x34);
                bytes.push(0x00);
            }
            "PHW" => {
                bytes.push(0x35);
                bytes.push(0x00);
            }
            "PHC" => {
                bytes.push(0x36);
                bytes.push(0x00);
            }
            "PLA" => {
                bytes.push(0x37);
                bytes.push(0x00);
            }
            "PLW" => {
                bytes.push(0x38);
                bytes.push(0x00);
            }
            "PLC" => {
                bytes.push(0x39);
                bytes.push(0x00);
            }
            "BRA" => {
                bytes.push(0x3A);
                let addr = labels.get(instruction[1]).expect("Label not found");
                bytes.push(*addr);
            }
            "BEQ" => {
                bytes.push(0x3B);
                let addr = labels.get(instruction[1]).expect("Label not found");
                bytes.push(*addr);
            }
            "BNE" => {
                bytes.push(0x3C);
                let addr = labels.get(instruction[1]).expect("Label not found");
                bytes.push(*addr);
            }
            "BGE" => {
                bytes.push(0x3D);
                let addr = labels.get(instruction[1]).expect("Label not found");
                bytes.push(*addr);
            }
            "BLT" => {
                bytes.push(0x3E);
                let addr = labels.get(instruction[1]).expect("Label not found");
                bytes.push(*addr);
            }
            "RET" => {
                bytes.push(0x3F);
                bytes.push(0x00);
            }
            "CALL" => {
                bytes.push(0x40);
                let addr = labels.get(instruction[1]).expect("Label not found");
                bytes.push(*addr);
            }
            "CEQ" => {
                bytes.push(0x41);
                let addr = labels.get(instruction[1]).expect("Label not found");
                bytes.push(*addr);
            }
            "CNE" => {
                bytes.push(0x42);
                let addr = labels.get(instruction[1]).expect("Label not found");
                bytes.push(*addr);
            }
            "CGE" => {
                bytes.push(0x43);
                let addr = labels.get(instruction[1]).expect("Label not found");
                bytes.push(*addr);
            }
            "CLT" => {
                bytes.push(0x44);
                let addr = labels.get(instruction[1]).expect("Label not found");
                bytes.push(*addr);
            }
            "SLA" => {
                bytes.push(0x45);
                bytes.push(0x00);
            }
            "SLW" => {
                bytes.push(0x46);
                bytes.push(0x00);
            }
            "SLC" => {
                bytes.push(0x47);
                bytes.push(0x00);
            }
            "SRA" => {
                bytes.push(0x48);
                bytes.push(0x00);
            }
            "SRW" => {
                bytes.push(0x49);
                bytes.push(0x00);
            }
            "SRC" => {
                bytes.push(0x4A);
                bytes.push(0x00);
            }
            _ => panic!("Unknown instruction"),
        }
        offset += 2;
    }
    return bytes;
}