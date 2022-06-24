use std::{
    env::args,
    fmt::Binary,
    ops::{Add, ControlFlow},
};

use bitvec::prelude::*;
mod globals;
use globals::*;
mod pseudo;
use pseudo::*;
mod flow;
use flow::*;

fn main() {
    let mut word;
    let path = "./output.bin";
    let capacity = std::fs::metadata(path).unwrap().len() as usize;
    let bytes = std::fs::read(path).unwrap();
    let mut binary_vec = Vec::with_capacity(capacity);
    binary_vec.push(Word(0));

    for pair in bytes.chunks_exact(2) {
        word = Word(u16::from_le_bytes([pair[0], pair[1]]));
        binary_vec.push(word);
    }

    let mut scope = CurrentBinaryScope::new(&binary_vec);

    let mut flowcontroller = FlowController::new();

    loop {
        scope.step();

        let address = scope.address.0; // stores initial address instead of final address
                                       // println!("{address:04x}");
                                       // println!("{:#04x}", scope.current_word.0);

        let flavor = get_instruction_flavor(&scope);
        let mut instruction = check_special_am(&get_instruction(flavor, &mut scope));

        match check_pseudo(instruction) {
            Some(pseudo) => instruction = pseudo,
            _ => (),
        }

        // check_for_flow(&mut flowcontroller, &mut scope, instruction);

        println!("{:04x}   {}       {instruction}", address, scope.used_words,);
    }
}

fn get_instruction(flavor: InstructionFlavor, scope: &mut CurrentBinaryScope) -> Instruction {
    let word = scope.current_word.0;
    let bits = word.view_bits::<Lsb0>();

    // println!("{:016b}", &bits[0..=15]);

    use InstructionFlavor::*;

    if flavor == ONE {
        let opcode = ONE_MAP.get(&bits[7..=9].load()).unwrap();
        let b = Bbit(bits[6]);
        let dest_reg = DestReg(bits[0..=3].load());
        let dam = match dest_reg.0 {
            SR => ADDRESS_MODE_SR_MAP.get(&bits[4..=5].load::<u8>()).unwrap(),
            ZR => ADDRESS_MODE_ZERO_MAP
                .get(&bits[4..=5].load::<u8>())
                .unwrap(),
            _ => ADDRESS_MODE_MAP.get(&bits[4..=5].load::<u8>()).unwrap(),
        };
        let dest_index = match dam {
            &AddressMode::IndirectIncrement => {
                if dest_reg.0 == PC {
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
        // println!("{:#x}", scope.current_word.0);
        // println!("{:016b}", &bits[0..=15]);

        let opcode = TWO_MAP.get(&bits[12..=15].load()).unwrap();

        let src_reg = SrcReg(bits[8..=11].load::<u8>().swap_bytes());

        let b = Bbit(bits[6]);

        let dest_reg = DestReg(bits[0..=3].load::<u8>().swap_bytes());

        let sam = match src_reg.0 {
            SR => ADDRESS_MODE_SR_MAP.get(&bits[4..=5].load::<u8>()).unwrap(),
            ZR => ADDRESS_MODE_ZERO_MAP
                .get(&bits[4..=5].load::<u8>())
                .unwrap(),
            _ => ADDRESS_MODE_MAP.get(&bits[4..=5].load::<u8>()).unwrap(),
        };

        let bool_int = match bits[7] {
            true => 1,
            false => 0,
        };

        let dam = match dest_reg.0 {
            SR => ADDRESS_MODE_SR_MAP.get(&bool_int).unwrap(),
            ZR => ADDRESS_MODE_ZERO_MAP.get(&bool_int).unwrap(),
            _ => ADDRESS_MODE_MAP.get(&bool_int).unwrap(),
        };

        let src_index = match sam {
            &AddressMode::IndirectIncrement => {
                if src_reg.0 == PC {
                    Some(Word(scope.get_next().0))
                } else {
                    None
                }
            }
            &AddressMode::Indexed | &AddressMode::AbsoluteAddressing => {
                Some(Word(scope.get_next().0))
            }
            _ => None,
        };
        let dest_index = match dam {
            &AddressMode::IndirectIncrement => {
                if dest_reg.0 == PC {
                    Some(Word(scope.get_next().0))
                } else {
                    None
                }
            }
            &AddressMode::Indexed | &AddressMode::AbsoluteAddressing => {
                Some(Word(scope.get_next().0))
            }
            _ => None,
        };
        Instruction::TWO {
            opcode: *opcode,
            src: src_reg,
            dam: *dam,
            b: b,
            sam: *sam,
            dest: dest_reg,
            src_index: src_index,
            dest_index: dest_index,
        }
    } else {
        // JMP

        let condition = JUMP_MAP.get(&bits[10..=12].load()).unwrap();

        // dont need to worry about sign extension since bitvec is amazing. Still, thanks for Retr0id and Stuckpixel for helping me out with learning how to do it

        let offset = Offset(((bits[0..=9].load::<i16>()) * 2) + 2); // These are all PC-relative jumps, adding twice the sign-extended offset to the PC, for a jump range of -1024 to +1022 (http://mspgcc.sourceforge.net/manual/x223.html)
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
