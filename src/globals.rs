use bitvec::{prelude::*, slice::BitSlice};

use core::fmt;
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    fmt::{write, Display},
    ops::Add,
};

use crate::{flow::Address, pseudo::PsuedoOpcode};
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

    pub static ref ADDRESS_MODE_SR_MAP: HashMap<u8, AddressMode> = {
        use AddressMode::*;
        let mut m = HashMap::new();
        m.insert(0b00, Direct);
        m.insert(0b01, AbsoluteAddressing);
        m.insert(0b10, Const4);
        m.insert(0b11, Const8);
        m
    };
    pub static ref ADDRESS_MODE_ZERO_MAP: HashMap<u8, AddressMode> = {
        use AddressMode::*;
        let mut m = HashMap::new();

        m.insert(0b00, Const0);
        m.insert(0b01, Const1);
        m.insert(0b10, Const2);
        m.insert(0b11, ConstNeg1);
        m
    };

}

pub const PC: u8 = 0; // Program Counter
pub const SP: u8 = 1; // Stack Pointer
pub const SR: u8 = 2; // Status Register
pub const ZR: u8 = 3; // Zero Register

pub struct CurrentBinaryScope<'w> {
    pub used_words: UsedWords, // stores words that were used for this current instruction
    pub address: Address,
    pub index: usize,
    pub current_word: Word,
    pub next_word: Word,
    pub vec: &'w Vec<Word>,
}

pub struct UsedWords(Vec<Word>);

impl fmt::Display for UsedWords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut new = String::new();

        for word in self.0.iter() {
            new += format!("{:04x}", word.0.swap_bytes()).as_str();
            new += " ";
        }

        write!(f, "{}", new.trim())
    }
}

impl CurrentBinaryScope<'_> {
    pub fn new(binary_vec: &Vec<Word>) -> CurrentBinaryScope {
        CurrentBinaryScope {
            used_words: UsedWords(Vec::new()),
            address: Address(0),
            index: 0,
            current_word: Word(0),
            next_word: Word(0),
            vec: binary_vec,
        }
    }
    pub fn step(&mut self) {
        self.used_words.0.clear();
        self.index += 1;
        self.update();
    }

    pub fn get_next(&mut self) -> Word {
        self.index += 1;
        self.used_words.0.push(self.vec[self.index]);
        self.vec[self.index]
    }

    pub fn update(&mut self) {
        self.address = Address::from_index(self.index);
        self.current_word = *self.vec.get(self.index).unwrap_or(&Word(0));
        self.next_word = *self.vec.get(self.index + 1).unwrap_or(&Word(0));
        self.used_words.0.push(self.vec[self.index]);
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
#[derive(Clone, Copy, Debug, PartialEq)]
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

    // SR
    AbsoluteAddressing,
    Const4,
    Const8,

    // ZR
    Const0,
    Const1,
    Const2,
    ConstNeg1,
}
#[derive(Clone, Copy, Debug)]
pub struct Indexing(pub u8);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DestReg(pub u8);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SrcReg(pub u8);
#[derive(Clone, Copy, Debug)]
pub struct Bbit(pub bool);

#[derive(Clone, Copy, Debug)]
pub struct Offset(pub i16);

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Invalid,
    JMP {
        condition: JmpOpcode,
        offset: Offset,
    },
    ONE {
        opcode: OneOpcode,
        b: Bbit,
        dam: AddressMode,
        dest: DestReg,
        dest_index: Option<Word>,
    },
    TWO {
        opcode: TwoOpcode,
        src: SrcReg,
        dam: AddressMode,
        b: Bbit,
        sam: AddressMode,
        dest: DestReg,
        src_index: Option<Word>,
        dest_index: Option<Word>,
    },
    TWO_BUT_WITH_A_SIGNED_WORD_I_HATE_RUST {
        opcode: TwoOpcode,
        src: SrcReg,
        dam: AddressMode,
        b: Bbit,
        sam: AddressMode,
        dest: DestReg,
        src_index: Option<SignedWord>,
        dest_index: Option<Word>,
    },
    PSEUDO {
        dest_index: Option<Word>,
        opcode: PsuedoOpcode,
        b: Bbit,
        dam: AddressMode,
        dest: Option<DestReg>,
    },
}
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct SignedWord(i16);

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
                let pos_or_neg;
                if offset >= 0 {
                    pos_or_neg = "+";
                } else {
                    pos_or_neg = "-";
                }
                write!(f, "{condition:?}    ${pos_or_neg}{:#x}", offset.abs())
            }
            Instruction::ONE {
                opcode,
                b,
                dam,
                dest,
                dest_index,
            } => {
                if dam == &AddressMode::IndirectIncrement && dest.0 == PC {
                    return write!(f, "{opcode:?}   {b} #{:#x}", dest_index.unwrap().0);
                }

                let indirect = match dam {
                    Indirect | IndirectIncrement => "@",
                    _ => "",
                };
                let increment = match dam {
                    IndirectIncrement => "+",
                    _ => "",
                };

                let offset = match dest_index {
                    Some(word) => format!("({:#x})", word.0),
                    _ => "".to_owned(),
                };

                let indexing = match dam {
                    Indexed => format!("({:#x})", dest_index.unwrap().0),
                    _ => "".to_owned(),
                };

                write!(
                    f,
                    "{opcode:?}{b}   {offset}{indirect}{indexing}{dest}{increment}"
                )
            }
            Instruction::TWO {
                opcode,
                src,
                dam,
                b,
                sam,
                dest,
                src_index,
                dest_index,
            } => {
                let indexing = match dam {
                    Indexed => format!("({:#x})", dest_index.unwrap().0),
                    _ => "".to_owned(),
                };

                if sam == &AddressMode::IndirectIncrement && src.0 == PC {
                    return write!(
                        f,
                        "{opcode:?}{b}    #{:#x}, {indexing}{dest}",
                        src_index.unwrap().0
                    );
                } else if sam == &AddressMode::AbsoluteAddressing {
                    return write!(f, "{opcode:?}{b}    &{:#x}, {dest}", src_index.unwrap().0);
                } else if dam == &AddressMode::AbsoluteAddressing {
                    //println!("welp");
                    return write!(f, "{opcode:?}{b}    {src}, &{:#x}", dest_index.unwrap().0);
                }
                let indirect = match sam {
                    Indirect | IndirectIncrement => "@",
                    _ => "",
                };
                let increment = match sam {
                    IndirectIncrement => "+",
                    _ => "",
                };

                let sindexing = match sam {
                    Indexed => format!("({:#x})", src_index.unwrap().0),
                    _ => "".to_owned(),
                };

                write!(
                    f,
                    "{opcode:?}{b}    {indirect}{sindexing}{src}{increment}, {indexing}{dest}"
                )
            }
            Instruction::PSEUDO {
                opcode,
                b,
                dam,
                dest,
                dest_index,
            } => {
                if dam == &AddressMode::AbsoluteAddressing {
                    return write!(f, "{opcode:?}{b}     &{:#x}", dest_index.unwrap().0);
                }
                let dest = match dest {
                    Some(dest) => format!("{dest}"),
                    _ => "".to_owned(),
                };

                write!(f, "{opcode:?}{b}    {dest}")
            }
            Instruction::Invalid => {
                write!(f, "Invalid Instruction!")
            }
            Instruction::TWO_BUT_WITH_A_SIGNED_WORD_I_HATE_RUST {
                opcode,
                src,
                dam,
                b,
                sam,
                dest,
                src_index,
                dest_index,
            } => {
                if sam == &AddressMode::IndirectIncrement && src.0 == PC {
                    return write!(f, "{opcode:?}{b}    #{:#x}, {dest}", src_index.unwrap().0);
                } else if sam == &AddressMode::AbsoluteAddressing {
                    return write!(f, "{opcode:?}{b}    &{:#x}, {dest}", src_index.unwrap().0);
                } else if dam == &AddressMode::AbsoluteAddressing {
                    //println!("welp");
                    return write!(f, "{opcode:?}{b}    {src}, &{:#x}", dest_index.unwrap().0);
                }
                let indirect = match sam {
                    Indirect | IndirectIncrement => "@",
                    _ => "",
                };
                let increment = match sam {
                    IndirectIncrement => "+",
                    _ => "",
                };
                write!(f, "{opcode:?}{b}    {indirect}{src}{increment}, {dest}")
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
                true => ".B",
                false => "",
            }
        )
    }
}

impl fmt::Display for SrcReg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = match self.0 {
            0 => "PC".to_owned(),
            1 => "SP".to_owned(),
            2 => "SR".to_owned(),
            3 => "ZR".to_owned(),
            num => format!("r{num}"),
        };

        write!(f, "{r}")
    }
}

impl fmt::Display for DestReg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = match self.0 {
            0 => "PC".to_owned(),
            1 => "SP".to_owned(),
            2 => "SR".to_owned(),
            num => format!("r{num}"),
        };
        write!(f, "{r}")
    }
}

pub fn check_special_am(instruction: &Instruction) -> Instruction {
    match instruction {
        Instruction::TWO {
            opcode,
            src,
            dam,
            b,
            sam,
            dest,
            src_index,
            dest_index,
        } => {
            let mut source = src;
            let mut source_index = src_index;
            let mut source_am = sam;

            match sam {
                &AddressMode::Const0 => {
                    source_am = &AddressMode::IndirectIncrement;
                    source = &SrcReg(PC);
                    source_index = &Some(Word(0));
                }
                &AddressMode::Const1 => {
                    source_am = &AddressMode::IndirectIncrement;
                    source = &SrcReg(PC);
                    source_index = &Some(Word(1));
                }
                &AddressMode::Const4 => {
                    source_am = &AddressMode::IndirectIncrement;
                    source = &SrcReg(PC);
                    source_index = &Some(Word(4));
                }
                &AddressMode::Const8 => {
                    source_am = &AddressMode::IndirectIncrement;
                    source = &SrcReg(PC);
                    source_index = &Some(Word(8));
                }
                &AddressMode::ConstNeg1 => {
                    source_am = &AddressMode::IndirectIncrement;
                    source = &SrcReg(PC);
                    let source_index = &Some(SignedWord(-1));

                    return Instruction::TWO_BUT_WITH_A_SIGNED_WORD_I_HATE_RUST {
                        opcode: *opcode,
                        src: *source,
                        dam: *dam,
                        b: *b,
                        sam: *source_am,
                        dest: *dest,
                        src_index: *source_index,
                        dest_index: *dest_index,
                    };
                }
                _ => (),
            }

            Instruction::TWO {
                opcode: *opcode,
                src: *source,
                dam: *dam,
                b: *b,
                sam: *source_am,
                dest: *dest,
                src_index: *source_index,
                dest_index: *dest_index,
            }
        }
        _ => *instruction,
    }
}
