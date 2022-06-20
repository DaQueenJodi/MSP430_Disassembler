use bitvec::prelude::*;
mod globals;
use globals::*;

fn main() {
    let mut word = 0;

    let bytes = std::fs::read("extras/welp.bin").unwrap();

    for pair in bytes.chunks_exact(2) {
        word = u16::from_le_bytes([pair[0], pair[1]]);
        let bits = word.view_bits::<Lsb0>();
        println!("{:#x}", word);
        println!("{}", &bits);
        let flavor = get_instruction_flavor(word);
        println!("{}", get_instruction(flavor, word));
        //return;
    }
}

fn get_instruction(flavor: InstructionFlavor, word: u16) -> Instruction {
    //let flavor = get_instruction_flavor(word);

    let bits = word.view_bits::<Lsb0>();

    use InstructionFlavor::*;

    if flavor == ONE {
        let opcode = ONE_MAP.get(&bits[7..=9].load()).unwrap();
        let b = Bbit(bits[6]);
        let dam = ADDRESS_MODE_MAP.get(&bits[4..=5].load()).unwrap();
        let dst_reg = DestReg(bits[0..=3].load::<u8>().swap_bytes());

        return Instruction::ONE {
            opcode: *opcode,
            b: b,
            dam: *dam,
            dest: dst_reg,
        };
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

        return Instruction::TWO {
            opcode: *opcode,
            src: src,
            dam: *dam,
            b: b,
            sam: *sam,
            dest: dest,
        };
    } else {
        // JMP

        let condition = JUMP_MAP.get(&bits[10..=12].load()).unwrap();
        let offset = Offset(bits[0..=9].load::<u16>().swap_bytes());

        return Instruction::JMP {
            condition: *condition,
            offset: offset,
        };
    }
}

fn get_instruction_flavor(word: u16) -> InstructionFlavor {
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
