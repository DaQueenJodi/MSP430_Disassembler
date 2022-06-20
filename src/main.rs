use std::{fmt::Binary, ops::Add};

use bitvec::prelude::*;
mod globals;
use globals::*;

fn main() {
    let mut scope = CurrentBinaryScope {
        index: 0,
        current_word: Word(0),
        next_word: Word(0),
        next_two_word: Word(0),
    };

    let mut word;
    let path = "extras/welp.bin";
    let capacity = std::fs::metadata(path).unwrap().len() as usize;
    let bytes = std::fs::read(path).unwrap();

    let mut BINARY_VEC = Vec::with_capacity(capacity);

    for pair in bytes.chunks_exact(2) {
        word = Word(u16::from_le_bytes([pair[0], pair[1]]));
        BINARY_VEC.push(word);
    }

    let mut result = None;
    loop {
        result = scope.step(&BINARY_VEC, result);
        let flavor = get_instruction_flavor(&scope);
        let instruction = get_instruction(flavor, &scope);
        println!("{instruction}");
    }
}

fn get_instruction(flavor: InstructionFlavor, scope: &CurrentBinaryScope) -> Instruction {
    // indexing controlls wether or not the function requires an extra word or 2 in order to fully be completed

    let word = scope.current_word.0;
    let bits = word.view_bits::<Lsb0>();

    use InstructionFlavor::*;

    println!("{:02b}", &bits[4..=5]);

    if flavor == ONE {
        let opcode = ONE_MAP.get(&bits[7..=9].load()).unwrap();
        let b = Bbit(bits[6]);
        let dam = ADDRESS_MODE_MAP.get(&bits[4..=5].load::<u8>()).unwrap();
        let dst_reg = DestReg(bits[0..=3].load());
        println!("{dam:?}");
        return (
            Instruction::ONE {
                opcode: *opcode,
                b: b,
                dam: *dam,
                dest: dst_reg,
            }
        );
    }
    if flavor == TWO {
        let opcode = TWO_MAP.get(&bits[12..=15].load()).unwrap();
        let src = SrcReg(bits[8..=11].load::<u8>().swap_bytes());
        let dam = ADDRESS_MODE_MAP
            .get(match bits[7] {
                true => &1,
                false => &0,
            })
            .unwrap();

        let b = Bbit(bits[6]);
        let sam = ADDRESS_MODE_MAP.get(&bits[4..=5].load()).unwrap();
        let dest = DestReg(bits[0..=3].load::<u8>().swap_bytes());

            Instruction::TWO {
                opcode: *opcode,
                src: src,
                dam: *dam,
                b: b,
                sam: *sam,
                dest: dest,
            }
    } else {
        // JMP

        let condition = JUMP_MAP.get(&bits[10..=12].load()).unwrap();
        let offset = Offset(bits[0..=9].load::<u16>().swap_bytes());

            Instruction::JMP {
                condition: *condition,
                offset: offset,
            }
    }
}

fn get_instruction_flavor(scope: &CurrentBinaryScope) -> InstructionFlavor {
    let word = scope.current_word.0;
    let bits = word.view_bits::<Lsb0>();
    use InstructionFlavor::*;
    if bits[10..16] == bits![0, 0, 1, 0, 0, 0] {
        ONE
    } else if bits[13..16] == bits![1, 0, 0] {
        JMP
    } else {
        TWO
    }
}
