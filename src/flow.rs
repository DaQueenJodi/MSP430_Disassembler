use std::{collections::HashSet, ops::Add};

use crate::{
    globals::{CurrentBinaryScope, Instruction, Offset, OneOpcode, Word},
    pseudo::PsuedoOpcode,
};
use crossbeam_queue::SegQueue;
use lazy_static::lazy_static;

/*
    LOGIC:
        search for CALL, RET, and JEQ/JNE/etc
        if CALL, save adress in a vector
        when RET, pop the call vector and go to the adress
        when JEQ/JNE/etc, save all posibilities to Queue vector
*/

pub fn check_for_flow(
    flowcontroller: &mut FlowController,
    scope: &mut CurrentBinaryScope,
    instruction: Instruction,
) {
    flowcontroller.add_to_blacklist(scope.address);

    match instruction {
        Instruction::JMP { condition, offset } => {
            flowcontroller.flow_add_to_queue_offset(scope, offset);
        }
        Instruction::PSEUDO { opcode, b, dest } => {
            if opcode == PsuedoOpcode::RET {
                flowcontroller.flow_return(scope);
            }
        }
        Instruction::ONE {
            opcode,
            b,
            dam,
            dest,
            dest_index,
        } => {
            if opcode == OneOpcode::CALL {
                flowcontroller.flow_call(scope, dest_index);
            }
        }

        _ => (),
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Address(pub u64);

impl Address {
    pub fn from_index(index: usize) -> Address {
        Address(
            ((index as u64 - 1 as u64) * 2)
                .try_into()
                .expect("for all the 128 bit machines ig"),
        )
    }
    pub fn to_index(&self) -> usize {
        ((self.0 / 16) + 1) as usize
    }
}

impl CurrentBinaryScope<'_> {
    pub fn jump_to_offset(&mut self, offset: Offset) {
        self.index = (Address::from_index(self.index) + offset.0).to_index();
        self.update();
    }
    pub fn jump_to_absolute(&mut self, address: Address) {
        self.index = address.to_index();
    }
}

impl Add<u16> for Address {
    type Output = Address;

    fn add(self, rhs: u16) -> Self::Output {
        Address(self.0 + rhs as u64)
    }
}
impl Add<usize> for Address {
    type Output = Address;

    fn add(self, rhs: usize) -> Self::Output {
        Address(self.0 + rhs as u64)
    }
}
impl Add<i16> for Address {
    type Output = Address;

    fn add(self, rhs: i16) -> Self::Output {
        Address(self.0 + rhs as u64)
    }
}

pub struct FlowController {
    pub BLACKLIST: HashSet<Address>,
    pub BRANCH_QUEUE: Vec<Address>,
    pub CALL_QUEUE: Vec<Address>,
}

impl FlowController {
    pub fn new() -> FlowController {
        FlowController {
            BLACKLIST: HashSet::new(),
            BRANCH_QUEUE: Vec::new(),
            CALL_QUEUE: Vec::new(),
        }
    }

    pub fn flow_add_to_queue_offset(&mut self, scope: &mut CurrentBinaryScope, offset: Offset) {
        self.BRANCH_QUEUE
            .push(Address::from_index(scope.index) + offset.0);
    }
    pub fn flow_call(&mut self, scope: &mut CurrentBinaryScope, dest_immediate: Option<Word>) {
        match dest_immediate {
            Some(word) => {
                self.CALL_QUEUE.push(Address::from_index(scope.index + 1));
                scope.jump_to_absolute(Address(word.0.into()));
            }
            _ => (),
        }
    }
    pub fn flow_return(&mut self, scope: &mut CurrentBinaryScope) {
        if !self.BRANCH_QUEUE.is_empty() {
            while self
                .BLACKLIST
                .contains(&Address(self.BRANCH_QUEUE.pop().unwrap().0))
            {}
        } else {
            scope.jump_to_absolute(Address(self.CALL_QUEUE.pop().unwrap().0));
        }
    }

    pub fn add_to_blacklist(&mut self, address: Address) {
        if self.BLACKLIST.contains(&address) {
            self.BLACKLIST.insert(address);
        }
    }
}
