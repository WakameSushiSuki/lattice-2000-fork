mod math;
mod compare;
mod logic;
mod shift;

use std::fs;
use std::env;

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
    let mut sp: u8 = 0xFF;
    let mut memory: Vec<u8> = vec![0; 256];

    while i < code.len() {
        let instr = vec![code[i], code[i+1]];
        i += 2;

        match instr[0] {
            0x00 => { // NOP
                
            }
            0x01 => { // ADA
                let res = math::add(reg_a, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                reg_a = res[0];
                
            }
            0x02 => { // ADW
                let res = math::add(reg_w, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                reg_w = res[0];
                
            }
            0x03 => { // ADC
                let res = math::add(reg_c, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                reg_c = res[0];
                
            }
            0x04 => { // SBA
                let res = math::sub(reg_a, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                reg_a = res[0];
                
            }
            0x05 => { // SBW
                let res = math::sub(reg_w, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                reg_w = res[0];
                
            }
            0x06 => { // SBC
                let res = math::sub(reg_c, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                reg_c = res[0];
                
            }
            0x07 => { // MLA
                reg_a = math::mul(reg_a, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                
            }
            0x08 => { // MLW
                reg_w = math::mul(reg_w, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                
            }
            0x09 => { // MLC
                reg_c = math::mul(reg_c, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                
            }
            0x0A => { // DVA
                reg_a = math::div(reg_a, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                
            }
            0x0B => { // DVW
                reg_w = math::div(reg_w, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                
            }
            0x0C => { // DVC
                reg_c = math::div(reg_c, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                
            }
            0x0D => { // LMA
                reg_a = math::lmod(reg_a, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                
            }
            0x0E => { // LMW
                reg_w = math::lmod(reg_w, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                
            }
            0x0F => { // LMC
                reg_c = math::lmod(reg_c, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                
            }
            0x10 => { // TAW
                reg_a = reg_w;
                
            }
            0x11 => { // TAC
                reg_a = reg_c;
                
            }
            0x12 => { // TWA
                reg_w = reg_a;
                
            }
            0x13 => { // TWC
                reg_w = reg_c;
                
            }
            0x14 => { // TCA
                reg_c = reg_a;
                
            }
            0x15 => { // TCW
                reg_c = reg_w;
                
            }
            0x16 => { // ICA
                reg_a = math::add(reg_a, 1)[0];
                
            }
            0x17 => { // ICW
                reg_w = math::add(reg_w, 1)[0];
                
            }
            0x18 => { // ICC
                reg_c = math::add(reg_c, 1)[0];
                
            }
            0x19 => { // DCA
                reg_a = math::sub(reg_a, 1)[0];
                
            }
            0x1A => { // DCW
                reg_w = math::sub(reg_w, 1)[0];
                
            }
            0x1B => { // DCC
                reg_c = math::sub(reg_c, 1)[0];
                
            }
            0x1C => { // CPA
                let res = compare::cmp(reg_a, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                eq = res[0];
                lt = res[1];
                gt = res[2];
                
            }
            0x1D => { // CPW
                let res = compare::cmp(reg_w, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                eq = res[0];
                lt = res[1];
                gt = res[2];
                
            }
            0x1E => { // CPC
                let res = compare::cmp(reg_c, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                eq = res[0];
                lt = res[1];
                gt = res[2];
                
            }
            0x1F => { // ORA
                reg_a = logic::lor(reg_a, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                
            }
            0x20 => { // ORW
                reg_w = logic::lor(reg_w, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                
            }
            0x21 => { // ORC
                reg_c = logic::lor(reg_c, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                
            }
            0x22 => { // ANA
                reg_a = logic::land(reg_a, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                
            }
            0x23 => { // ANW
                reg_w = logic::land(reg_w, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                
            }
            0x24 => { // ANC
                reg_c = logic::land(reg_c, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                
            }
            0x25 => { // XRA
                reg_a = logic::lxor(reg_a, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                
            }
            0x26 => { // XRW
                reg_w = logic::lxor(reg_w, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                
            }
            0x27 => { // XRC
                reg_c = logic::lxor(reg_c, match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0,
                });
                
            }
            0x28 => { // NTA
                reg_a = logic::lnot(reg_a);
                
            }
            0x29 => { // NTW
                reg_w = logic::lnot(reg_w);
                
            }
            0x2A => { // NTC
                reg_c = logic::lnot(reg_c);
                
            }
            0x2B => { // LDA
                reg_a = match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0
                };
                
            }
            0x2C => { // LDW
                reg_w = match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0
                };
                
            }
            0x2D => { // LDC
                reg_c = match memory.get(instr[1] as usize) {
                    Some(x) => *x,
                    None => 0
                };
                
            }
            0x2E => { // STA
                memory[instr[1] as usize] = reg_a;
                
            }
            0x2F => { // STW
                memory[instr[1] as usize] = reg_w;
                
            }
            0x30 => { // STC
                memory[instr[1] as usize] = reg_c;
                
            }
            0x31 => { // LAI
                reg_a = instr[1];
                
            }
            0x32 => { // LWI
                reg_w = instr[1];
                
            }
            0x33 => { // LCI
                reg_c = instr[1];
                
            }
            0x34 => { // PHA
                memory[sp as usize] = reg_a;
                sp = math::sub(sp, 1)[0];
                
            }
            0x35 => { // PHW
                memory[sp as usize] = reg_w;
                sp = math::sub(sp, 1)[0];
                
            }
            0x36 => { // PHC
                memory[sp as usize] = reg_c;
                sp = math::sub(sp, 1)[0];
                
            }
            0x37 => { // PLA
                reg_a = memory[sp as usize];
                memory[sp as usize] = 0x00;
                sp = math::add(sp, 1)[0];
                
            }
            0x38 => { // PLW
                reg_w = memory[sp as usize];
                memory[sp as usize] = 0x00;
                sp = math::add(sp, 1)[0];
                
            }
            0x39 => { // PLC
                reg_c = memory[sp as usize];
                memory[sp as usize] = 0x00;
                sp = math::add(sp, 1)[0];
                
            }
            0x3A => {
                i = instr[1] as usize;
            }
            0x3B => {
                if eq == 1 {
                    i = instr[1] as usize;
                }
            }
            0x3C => {
                if eq == 0 {
                    i = instr[1] as usize;
                }
            }
            0x3D => {
                if gt == 1 {
                    i = instr[1] as usize;
                }
            }
            0x3E => {
                if lt == 1 {
                    i = instr[1] as usize;
                }
            }
            0x3F => {
                i = memory[sp as usize] as usize;
                memory[sp as usize] = 0x00;
                sp = math::add(sp, 1)[0];
            }
            0x40 => {
                sp = math::sub(sp, 1)[0];
                memory[sp as usize] = i as u8;
                i = instr[1] as usize;
            }
            0x41 => {
                if eq == 1 {
                    sp = math::sub(sp, 1)[0];
                    memory[sp as usize] = i as u8;
                    i = instr[1] as usize;
                }
            }
            0x42 => {
                if eq == 0 {
                    sp = math::sub(sp, 1)[0];
                    memory[sp as usize] = i as u8;
                    i = instr[1] as usize;
                }
            }
            0x43 => {
                if gt == 1 {
                    sp = math::sub(sp, 1)[0];
                    memory[sp as usize] = i as u8;
                    i = instr[1] as usize;
                }
            }
            0x44 => {
                if lt == 1 {
                    sp = math::sub(sp, 1)[0];
                    memory[sp as usize] = i as u8;
                    i = instr[1] as usize;
                }
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