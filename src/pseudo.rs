use core::fmt;

use crate::globals::{AddressMode, Bbit, DestReg, Instruction, TwoOpcode, PC, SP, SR};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PsuedoOpcode {
    NOP,
    POP,

    BR,
    RET,

    CLRC,
    SETC,
    CLRZ,
    SETZ,
    CLRN,
    SETN,
    DINT,
    EINT,

    RLA,
    RLC,

    INV,
    CLR,
    TST,

    DEC,
    DECD,
    INC,
    INCD,

    ADC,
    DADC,
    SBC,
}

pub fn check_pseudo(instruction: Instruction) -> Option<Instruction> {
    {
        use AddressMode::*;
        use Instruction::*;
        use PsuedoOpcode::*;
        use TwoOpcode::*;
        match instruction {
            TWO {
                opcode,
                src,
                dam,
                b,
                sam,
                dest,
                src_index,
                dest_index,
            } => match opcode {
                MOV => {
                    if src.0 == PC && sam == IndirectIncrement && src_index.unwrap().0 == 0 {
                        return Some(PSEUDO {
                            dest_index: dest_index,
                            opcode: CLR,
                            dam: dam,
                            b: b,
                            dest: Some(dest),
                        });
                    }

                    if dest.0 == src.0 && sam == Direct && dam == Direct {
                        return Some(PSEUDO {
                            dest_index: dest_index,
                            opcode: NOP,
                            dam: dam,
                            b: b,
                            dest: None,
                        });
                    }
                    if sam == IndirectIncrement && src.0 == SP {
                        if dest.0 == PC {
                            return Some(PSEUDO {
                                opcode: RET,
                                dam: dam,
                                b: b,
                                dest: None,
                                dest_index: dest_index,
                            });
                        } else {
                            return Some(PSEUDO {
                                dest_index: dest_index,
                                opcode: POP,
                                dam: dam,
                                b: b,
                                dest: Some(dest),
                            });
                        }
                    }
                    if dest.0 == PC && dam == Direct {
                        return Some(PSEUDO {
                            dest_index: dest_index,
                            opcode: BR,
                            dam: dam,
                            b: b,
                            dest: Some(DestReg(src.0)),
                        });
                    }
                    None
                }
                BIC => {
                    if src.0 == PC && sam == IndirectIncrement && dest.0 == SR {
                        // BIC @pc+, SR
                        match src_index.unwrap().0 {
                            1 => {
                                return Some(PSEUDO {
                                    dest_index: dest_index,
                                    dam: dam,
                                    opcode: CLRC,
                                    b: b,
                                    dest: None,
                                })
                            }
                            2 => {
                                return Some(PSEUDO {
                                    dest_index: dest_index,
                                    dam: dam,
                                    opcode: CLRZ,
                                    b: b,
                                    dest: None,
                                })
                            }
                            4 => {
                                return Some(PSEUDO {
                                    dest_index: dest_index,
                                    opcode: CLRN,
                                    dam: dam,
                                    b: b,
                                    dest: None,
                                })
                            }
                            8 => {
                                return Some(PSEUDO {
                                    dest_index: dest_index,
                                    dam: dam,
                                    opcode: DINT,
                                    b: b,
                                    dest: None,
                                })
                            }
                            _ => return None,
                        }
                    }
                    None
                }
                BIS => {
                    if src.0 == PC && sam == IndirectIncrement && dest.0 == SR {
                        // BIC @pc+, SR
                        match src_index.unwrap().0 {
                            1 => {
                                return Some(PSEUDO {
                                    dest_index: dest_index,
                                    dam: dam,
                                    opcode: SETC,
                                    b: b,
                                    dest: None,
                                })
                            }
                            2 => {
                                return Some(PSEUDO {
                                    dest_index: dest_index,
                                    dam: dam,
                                    opcode: SETZ,
                                    b: b,
                                    dest: None,
                                })
                            }
                            4 => {
                                return Some(PSEUDO {
                                    dest_index: dest_index,
                                    dam: dam,
                                    opcode: SETN,
                                    b: b,
                                    dest: None,
                                })
                            }
                            8 => {
                                return Some(PSEUDO {
                                    dest_index: dest_index,
                                    dam: dam,
                                    opcode: EINT,
                                    b: b,
                                    dest: None,
                                })
                            }
                            _ => return None,
                        }
                    }
                    None
                }
                ADD => {
                    if dest.0 == src.0 {
                        return Some(PSEUDO {
                            dest_index: dest_index,
                            dam: dam,
                            opcode: RLA,
                            b: b,
                            dest: Some(dest),
                        });
                    }
                    None
                }
                ADDC => {
                    if dest.0 == src.0 {
                        return Some(PSEUDO {
                            dest_index: dest_index,
                            dam: dam,
                            opcode: RLA,
                            b: b,
                            dest: Some(dest),
                        });
                    }
                    None
                }
                XOR => {
                    // TODO signdedness
                    // if src.0 == PC && sam == IndirectIncrement && src_index.unwrap().0 == -1 {
                    //     return Some(PSEUDO{INV, b, Some(dest)));
                    // }
                    None
                }
                CMP => {
                    if src.0 == PC && sam == IndirectIncrement && src_index.unwrap().0 == 0 {
                        return Some(PSEUDO {
                            dest_index: dest_index,
                            dam: dam,
                            opcode: TST,
                            b: b,
                            dest: Some(dest),
                        });
                    }
                    None
                }
                SUB => {
                    if src.0 == PC && sam == IndirectIncrement && src_index.unwrap().0 == 1 {
                        return Some(PSEUDO {
                            dest_index: dest_index,
                            dam: dam,
                            opcode: DEC,
                            b: b,
                            dest: Some(dest),
                        });
                    }
                    None
                }
                ADD => {
                    if src.0 == PC && sam == IndirectIncrement && src_index.unwrap().0 == 1 {
                        return Some(PSEUDO {
                            dest_index: dest_index,
                            dam: dam,
                            opcode: INC,
                            b: b,
                            dest: Some(dest),
                        });
                    }
                    None
                }

                _ => None,
            },
            _ => None,
        }
    }
}
