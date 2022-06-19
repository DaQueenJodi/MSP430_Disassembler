use bitvec::{prelude::*, slice::BitSlice};
use lazy_static::lazy_static;
use std::collections::HashMap;
lazy_static! {
    pub static ref JUMP_MAP: HashMap<&'static BitSlice, JmpOpcode> = {
        use JmpOpcode::*;
        let mut m = HashMap::new();
        m.insert(bits![0, 0, 0], JNE);
        m.insert(bits![0, 0, 1], JEQ);
        m.insert(bits![0, 1, 0], JLO);
        m.insert(bits![0, 1, 1], JHS);
        m.insert(bits![1, 0, 0], JN);
        m.insert(bits![1, 0, 1], JGE);
        m.insert(bits![1, 1, 0], JL);
        m.insert(bits![1, 1, 1], JMP);
        m
    };
    pub static ref ONE_MAP: HashMap<&'static BitSlice, OneOpcode> = {
        use OneOpcode::*;
        let mut m = HashMap::new();
        m.insert(bits![0, 0, 0], RRC);
        m.insert(bits![0, 0, 1], SWPB);
        m.insert(bits![0, 1, 0], RRA);
        m.insert(bits![0, 1, 1], SXT);
        m.insert(bits![1, 0, 0], PUSH);
        m.insert(bits![1, 0, 1], CALL);
        m.insert(bits![1, 1, 0], RETI);
        m
    };
    pub static ref TWO_MAP: HashMap<&'static BitSlice, TwoOpcode> = {
        use TwoOpcode::*;
        let mut m = HashMap::new();
        m.insert(bits![0, 1, 0, 0], MOV);
        m.insert(bits![0, 1, 0, 1], ADD);
        m.insert(bits![0, 1, 1, 0], ADDC);
        m.insert(bits![0, 1, 1, 1], SUBC);
        m.insert(bits![1, 0, 0, 0], SUB);
        m.insert(bits![1, 0, 0, 1], CMP);
        m.insert(bits![1, 0, 1, 0], DADD);
        m.insert(bits![1, 0, 1, 1], BIT);
        m.insert(bits![1, 1, 0, 0], BIC);
        m.insert(bits![1, 1, 0, 1], BIS);
        m.insert(bits![1, 1, 1, 0], XOR);
        m.insert(bits![1, 1, 1, 1], AND);
        m
    };
    pub static ref ADDRESS_MODE_MAP: HashMap<&'static BitSlice, AddressMode> = {
        use AddressMode::*;
        let mut m = HashMap::new();
        // 2 bit for src)
        m.insert(bits![0, 0], Direct);
        m.insert(bits![0, 1], Indexed);
        m.insert(bits![1, 0], Indirect);
        m.insert(bits![1, 1], IndirectIncrement);


        // 1 bit (for dest)
        m.insert(bits![0], Direct);
        m.insert(bits![1], Indirect);

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
