use bitvec::{prelude::*, slice::BitSlice};
use core::fmt;
use lazy_static::lazy_static;
use std::{collections::HashMap, fmt::write};
lazy_static! {
    pub static ref JUMP_MAP: HashMap<u8, JmpOpcode> = {
        use JmpOpcode::*;
        let mut m = HashMap::new();
        m.insert(0b000, JNE);
        m.insert(0b001, JEQ);
        m.insert(0b010, JLO);
        m.insert(0b011, JHS);
        m.insert(0b100, JN);
        m.insert(0b101, JGE);
        m.insert(0b110, JL);
        m.insert(0b111, JMP);
        m
    };
    pub static ref ONE_MAP: HashMap<u8, OneOpcode> = {
        use OneOpcode::*;
        let mut m = HashMap::new();
        m.insert(0b000, RRC);
        m.insert(0b001, SWPB);
        m.insert(0b010, RRA);
        m.insert(0b011, SXT);
        m.insert(0b100, PUSH);
        m.insert(0b101, CALL);
        m.insert(0b110, RETI);
        m
    };
    pub static ref TWO_MAP: HashMap<u8, TwoOpcode> = {
        use TwoOpcode::*;
        let mut m = HashMap::new();
        m.insert(0b0100, MOV);
        m.insert(0b0101, ADD);
        m.insert(0b0110, ADDC);
        m.insert(0b0111, SUBC);
        m.insert(0b1000, SUB);
        m.insert(0b1001, CMP);
        m.insert(0b1010, DADD);
        m.insert(0b1011, BIT);
        m.insert(0b1100, BIC);
        m.insert(0b1101, BIS);
        m.insert(0b1110, XOR);
        m.insert(0b1111, AND);
        m
    };
    pub static ref ADDRESS_MODE_MAP: HashMap<u8, AddressMode> = {
        use AddressMode::*;
        let mut m = HashMap::new();
        // 2 bit for src)
        m.insert(0b00, Direct);
        m.insert(0b01, Indexed);
        m.insert(0b10, Indirect);
        m.insert(0b11, IndirectIncrement);
        m
    };
}

pub struct CurrentBinaryScope {
    pub index: usize,
    pub current_word: Word,
    pub next_word: Word,
    pub next_two_word: Word,
}

impl CurrentBinaryScope {
    // returns 0/1/2 depending on what offset of `vec` does not exist
    pub fn step(&mut self, vec: &Vec<Word>, result: Option<u8>) -> Option<u8> {
        self.index += 1;

        if result == Some(1) {
            panic!("welp");
        }

        self.current_word = match vec.get(self.index) {
            Some(word) => *word,
            _ => return Some(0),
        };

        if result == Some(2) {
            panic!("welp");
        }

        self.next_word = match vec.get(self.index + 1) {
            Some(word) => *word,
            _ => return Some(1),
        };
        self.next_two_word = match vec.get(self.index + 2) {
            Some(word) => *word,
            _ => return Some(2),
        };
        None
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Word(pub u16);

#[derive(Clone, Copy, Debug)]
pub enum JmpOpcode {
    JNE,
    JEQ,
    JLO,
    JHS,
    JN,
    JGE,
    JL,
    JMP,
}
#[derive(Clone, Copy, Debug)]
pub enum OneOpcode {
    RRC,
    SWPB,
    RRA,
    SXT,
    PUSH,
    CALL,
    RETI,
}
#[derive(Clone, Copy, Debug)]
pub enum TwoOpcode {
    MOV,
    ADD,
    ADDC,
    SUBC,
    SUB,
    CMP,
    DADD,
    BIT,
    BIC,
    BIS,
    XOR,
    AND,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AddressMode {
    Direct,            // Rn
    Indexed,           // (offset)Rn
    Indirect,          // @Rn
    IndirectIncrement, // @Rn+
}
#[derive(Clone, Copy, Debug)]
pub struct Indexing(pub u8);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DestReg(pub u8);
#[derive(Clone, Copy, Debug)]
pub struct SrcReg(pub u8);
#[derive(Clone, Copy, Debug)]
pub struct Bbit(pub bool);

#[derive(Clone, Copy, Debug)]
pub struct Offset(pub u16);

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    None,
    JMP {
        condition: JmpOpcode,
        offset: Offset,
    },
    ONE {
        opcode: OneOpcode,
        b: Bbit,
        dam: AddressMode,
        dest: DestReg,
    },
    TWO {
        opcode: TwoOpcode,
        src: SrcReg,
        dam: AddressMode,
        b: Bbit,
        sam: AddressMode,
        dest: DestReg,
    },
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum InstructionFlavor {
    JMP,
    ONE,
    TWO,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> fmt::Result {
        use AddressMode::*;
        match self {
            Instruction::JMP {
                condition, offset, ..
            } => {
                let offset = offset.0;
                write!(f, "{condition:?} {offset:#x}")
            }
            Instruction::ONE {
                opcode,
                b,
                dam,
                dest,
            } => {
                if dam == &AddressMode::IndirectIncrement && dest == &DestReg(0) {
                    //write!(f, "{opcode:?}{b} ")
                }

                let indirect = match dam {
                    Indirect | IndirectIncrement => "@",
                    _ => "",
                };
                let increment = match dam {
                    IndirectIncrement => "+",
                    _ => "",
                };
                write!(f, "{opcode:?}{b}  {indirect}{dest}{increment}")
            }
            Instruction::TWO {
                opcode,
                src,
                dam,
                b,
                sam,
                dest,
            } => {
                let indirect = match sam {
                    Indirect | IndirectIncrement => "@",
                    _ => "",
                };
                let increment = match sam {
                    IndirectIncrement => "+",
                    _ => "",
                };
                write!(f, "{opcode:?}{b}  {indirect}{src}{increment}, {dest}")
            }
            Instruction::None => {
                write!(f, "Invalid Instruction!")
            }
        }
    }
}

impl fmt::Display for Bbit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                true => ".b",
                false => "",
            }
        )
    }
}

impl fmt::Display for SrcReg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r{}", self.0)
    }
}

impl fmt::Display for DestReg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r{}", self.0)
    }
}
