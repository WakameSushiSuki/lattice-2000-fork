mod math;
mod compare;
mod logic;

use std::arch::x86_64;
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
    let mut gt: u8 = 0;
    let mut lt: u8 = 0;
    let mut eq: u8 = 0;
    let mut memory: HashMap<u8, u8> = HashMap::new();

    while i < code.len() {
        let instr = vec![code[i], code[i+1]];

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
            0x16 => { // ICA
                reg_a = math::add(reg_a, 1)[0];
                i = pcinc(i);
            }
            0x17 => { // ICW
                reg_w = math::add(reg_w, 1)[0];
                i = pcinc(i);
            }
            0x18 => { // ICC
                reg_c = math::add(reg_c, 1)[0];
                i = pcinc(i);
            }
            0x19 => { // DCA
                reg_a = math::sub(reg_a, 1)[0];
                i = pcinc(i);
            }
            0x1A => { // DCW
                reg_w = math::sub(reg_w, 1)[0];
                i = pcinc(i);
            }
            0x1B => { // DCC
                reg_c = math::sub(reg_c, 1)[0];
                i = pcinc(i);
            }
            0x1C => { // CPA
                let res = compare::cmp(reg_a, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                eq = res[0];
                lt = res[1];
                gt = res[2];
                i = pcinc(i);
            }
            0x1D => { // CPW
                let res = compare::cmp(reg_w, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                eq = res[0];
                lt = res[1];
                gt = res[2];
                i = pcinc(i);
            }
            0x1E => { // CPC
                let res = compare::cmp(reg_c, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                eq = res[0];
                lt = res[1];
                gt = res[2];
                i = pcinc(i);
            }
            0x1F => { // ORA
                reg_a = logic::lor(reg_a, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                i = pcinc(i);
            }
            0x20 => { // ORW
                reg_w = logic::lor(reg_w, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                i = pcinc(i);
            }
            0x21 => { // ORC
                reg_c = logic::lor(reg_c, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                i = pcinc(i);
            }
            0x22 => { // ANA
                reg_a = logic::land(reg_a, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                i = pcinc(i);
            }
            0x23 => { // ANW
                reg_w = logic::land(reg_w, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                i = pcinc(i);
            }
            0x24 => { // ANC
                reg_c = logic::land(reg_c, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                i = pcinc(i);
            }
            0x25 => { // XRA
                reg_a = logic::lxor(reg_a, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                i = pcinc(i);
            }
            0x26 => { // XRW
                reg_w = logic::lxor(reg_w, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                i = pcinc(i);
            }
            0x27 => { // XRC
                reg_c = logic::lxor(reg_c, match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0,
                });
                i = pcinc(i);
            }
            0x28 => { // NTA
                reg_a = logic::lnot(reg_a);
                i = pcinc(i);
            }
            0x29 => { // NTW
                reg_w = logic::lnot(reg_w);
                i = pcinc(i);
            }
            0x2A => { // NTC
                reg_c = logic::lnot(reg_c);
                i = pcinc(i);
            }
            0x2B => { // LDA
                reg_a = match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0
                };
                i = pcinc(i);
            }
            0x2C => { // LDW
                reg_w = match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0
                };
                i = pcinc(i);
            }
            0x2D => { // LDC
                reg_c = match memory.get(&instr[1]) {
                    Some(x) => *x,
                    None => 0
                };
                i = pcinc(i);
            }
            0x2E => { // STA
                memory.insert(instr[1], reg_a);
                i = pcinc(i);
            }
            0x2F => { // STW
                memory.insert(instr[1], reg_w);
                i = pcinc(i);
            }
            0x30 => { // STC
                memory.insert(instr[1], reg_c);
                i = pcinc(i);
            }
            0x31 => { // LAI
                reg_a = instr[1];
                i = pcinc(i)
            }
            0x31 => { // LWI
                reg_w = instr[1];
                i = pcinc(i)
            }
            0x31 => { // LCI
                reg_c = instr[1];
                i = pcinc(i)
            }
            0xFF => { // HLT
                break;
            }
            _ => {
                println!("Unknown instruction: {}", instr[0]);
                break;
            }
        }
    }
}

fn pcinc(i: usize) -> usize {
    return i + 2;
}