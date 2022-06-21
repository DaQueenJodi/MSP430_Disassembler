use std::{env::args, fmt::Binary, ops::Add};

use bitvec::prelude::*;
mod globals;
use globals::*;

fn main() {
    let mut word;
    let path = "extras/welp.bin";
    let capacity = std::fs::metadata(path).unwrap().len() as usize;
    let bytes = std::fs::read(path).unwrap();
    let mut binary_vec = Vec::with_capacity(capacity);
    binary_vec.push(Word(0));

    for pair in bytes.chunks_exact(2) {
        word = Word(u16::from_le_bytes([pair[0], pair[1]]));
        binary_vec.push(word);
    }

    let mut scope = CurrentBinaryScope {
        index: (0),
        current_word: Word(0),
        next_word: Word(0),
        vec: &binary_vec,
    };

    let mut result = None;
    loop {
        result = scope.step(result);
        println!("{:#x}", scope.current_word.0);

        let flavor = get_instruction_flavor(&scope);
        let instruction = get_instruction(flavor, &mut scope);

        println!(
            "{:04x} {:x} {instruction}",
            (scope.index - 1) * 8,
            scope.current_word.0.swap_bytes()
        );
    }
}

fn get_instruction(flavor: InstructionFlavor, scope: &mut CurrentBinaryScope) -> Instruction {
    // indexing controlls wether or not the function requires an extra word or 2 in order to fully be completed

    let word = scope.current_word.0;
    let bits = word.view_bits::<Lsb0>();

    use InstructionFlavor::*;

    if flavor == ONE {
        let opcode = ONE_MAP.get(&bits[7..=9].load()).unwrap();
        let b = Bbit(bits[6]);
        let dam = ADDRESS_MODE_MAP.get(&bits[4..=5].load::<u8>()).unwrap();
        let dest_reg = DestReg(bits[0..=3].load());
        let dest_index = match dam {
            &AddressMode::IndirectIncrement => {
                if dest_reg == DestReg(0) {
                    Some(Word(scope.get_next().0))
                } else {
                    None
                }
            }
            &AddressMode::Indexed => Some(Word(scope.get_next().0.swap_bytes())),
            _ => None,
        };
        return (Instruction::ONE {
            opcode: *opcode,
            b: b,
            dam: *dam,
            dest: dest_reg,
            dest_index: dest_index,
        });
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
        let src_index = match sam {
            &AddressMode::IndirectIncrement => {
                if src == SrcReg(0) {
                    Some(Word(scope.get_next().0))
                } else {
                    None
                }
            }
            &AddressMode::Indexed => Some(Word(scope.get_next().0.swap_bytes())),
            _ => None,
        };
        let dest_index = match dam {
            &AddressMode::IndirectIncrement => {
                if dest == DestReg(0) {
                    Some(Word(scope.get_next().0))
                } else {
                    None
                }
            }
            &AddressMode::Indexed => Some(Word(scope.get_next().0.swap_bytes())),
            _ => None,
        };
        Instruction::TWO {
            opcode: *opcode,
            src: src,
            dam: *dam,
            b: b,
            sam: *sam,
            dest: dest,
            src_index: src_index,
            dest_index: dest_index,
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
