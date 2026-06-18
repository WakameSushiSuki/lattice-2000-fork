mod math;
mod compare;

use std::fs;
use std::env;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code: Vec<u8> = fs::read(args[1].clone()).expect("Unable to read file");
    let mut i: usize = 0;
    let mut reg_a: u8 = 0;
    let mut reg_w: u8 = 0;
    let mut reg_c: u8 = 0;
    let mut memory: HashMap<u8, u8> = HashMap::new();

    while i < code.len() {
        let instr = vec![code[i], code[i+1], code[i+2], code[i+3]];

        match instr[0] {
            0x00 => { // NOP
                i += 4;
            }
            0x01 => { // ADA
                let res = math::add(reg_a, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                reg_a = res[0];
                i = pcinc(i);
            }
            0x02 => { // ADW
                let res = math::add(reg_w, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                reg_w = res[0];
                i = pcinc(i);
            }
            0x03 => { // ADC
                let res = math::add(reg_c, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                reg_c = res[0];
                i = pcinc(i);
            }
            0x04 => { // SBA
                let res = math::sub(reg_a, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                reg_a = res[0];
                i = pcinc(i);
            }
            0x05 => { // SBW
                let res = math::sub(reg_w, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                reg_w = res[0];
                i = pcinc(i);
            }
            0x06 => { // SBC
                let res = math::sub(reg_c, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                reg_c = res[0];
                i = pcinc(i);
            }
            0x07 => { // MLA
                reg_a = math::mul(reg_a, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                i = pcinc(i);
            }
            0x08 => { // MLW
                reg_w = math::mul(reg_w, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                i = pcinc(i);
            }
            0x09 => { // MLC
                reg_c = math::mul(reg_c, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                i = pcinc(i);
            }
            0x0A => { // DVA
                reg_a = math::div(reg_a, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                i = pcinc(i);
            }
            0x0B => { // DVW
                reg_w = math::div(reg_w, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                i = pcinc(i);
            }
            0x0C => { // DVC
                reg_c = math::div(reg_c, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                i = pcinc(i);
            }
            0x0D => { // LMA
                reg_a = math::lmod(reg_a, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                i = pcinc(i);
            }
            0x0E => { // LMW
                reg_w = math::lmod(reg_w, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                i = pcinc(i);
            }
            0x0F => { // LMC
                reg_c = math::lmod(reg_c, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                i = pcinc(i);
            }
            0x10 => { // TAW
                reg_a = reg_w;
                i = pcinc(i);
            }
            0x11 => { // TAC
                reg_a = reg_c;
                i = pcinc(i);
            }
            0x12 => { // TWA
                reg_w = reg_a;
                i = pcinc(i);
            }
            0x13 => { // TWC
                reg_w = reg_c;
                i = pcinc(i);
            }
            0x14 => { // TCA
                reg_c = reg_a;
                i = pcinc(i);
            }
            0x15 => { // TCW
                reg_c = reg_w;
                i = pcinc(i);
            }
            _ => {
                println!("Unknown instruction: {}", instr[0]);
                break;
            }
        }
    }
}

fn pcinc(i: usize) -> usize {
    let inc = math::add(i as u8, 4);
    return inc[0] as usize;
}