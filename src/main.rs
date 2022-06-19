#[macro_use]
extern crate lazy_static;
use bitvec::prelude::*;
use std::borrow::Borrow;
use std::io::Read;
use std::{
    fs::{self, File},
    path::Path,
};
mod globals;
use globals::*;


fn main() {
    //let x =  get_file_as_bytes(Path::new("extras/welp.bin"));
    //let x: Vec<u16> = vec![0x043c];
    //let x = x[0];
    //let x = 0x043c;
    let instruction: u16 = 0x013c;
    let bits = instruction.view_bits::<Msb0>();
    //println!("{bits:b}");
    println!("{:b}", &bits);
    println!("{:b}", &bits[13..16]);
}

fn get_file_as_bytes(path: &Path) -> Vec<u8> {
    let mut f = File::open(&path).expect("no file found");
    let metadata = fs::metadata(&path).expect("unable to read metadata");
    let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}


fn get_instruction(flavor: InstructionFlavor, word: u16) {
    let flavor = get_instruction_flavor(word);
    
    let bits = word.view_bits::<Msb0>();

    use InstructionFlavor::*;

    if flavor == ONE {
        let opcode = ONE_MAP.get(&bits[7..10]).unwrap();
    }
}

fn get_instruction_flavor(word: u16) -> InstructionFlavor {
    let bits = word.view_bits::<Msb0>();
    use InstructionFlavor::*;
    if bits[10..16] == bits![0, 0, 1, 0, 0, 0] {
        ONE
    } else if bits[13..16] == bits![1, 0, 0] {
        JMP
    } else {
        TWO
    }
}

// stolen from Maddie
struct U16Iter<I: Iterator<Item = u8>>(I);

impl<I: Iterator<Item = u8>> Iterator for U16Iter<I> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        let hi = self.0.next()?;
        let lo = self.0.next()?;

        Some((hi as u16) << 8 | lo as u16)
    }
}
