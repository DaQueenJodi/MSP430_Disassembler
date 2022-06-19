use bitvec::{prelude::*, slice::BitSlice};
use lazy_static::lazy_static;
use std::collections::HashMap;
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
    pub static ref TWO_MAP: HashMap<&'static BitSlice, TwoOpcode> = {
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


        // 1 bit (for dest)
        //m.insert(bits![0], Direct);
        //m.insert(bits![1], Indirect);

        m
    };
}

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

pub enum OneOpcode {
    RRC,
    SWPB,
    RRA,
    SXT,
    PUSH,
    CALL,
    RETI,
}
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

pub enum Opcode {
    JMP(JmpOpcode),
    ONE(OneOpcode),
    TWO(TwoOpcode),
}

pub enum AddressMode {
    Direct,            // Rn
    Indexed,           // (offset)Rn
    Indirect,          // @Rn
    IndirectIncrement, // @Rn+
}

struct DestReg(u8);
struct SrcReg(u8);
struct Bbit(bool);

pub enum Instruction {
    JMP {
        condition: Opcode,
        offset: DestReg,
    },
    ONE {
        opcode: Opcode,
        b: Bbit,
        dam: AddressMode,
        dest: DestReg,
    },
    TWO {
        opcode: Opcode,
        src: SrcReg,
        dam: AddressMode,
        b: Bbit,
        sam: AddressMode,
        dest: DestReg,
    },
}

#[derive(PartialEq, Eq)]
pub enum InstructionFlavor {
    JMP,
    ONE,
    TWO,
}
