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
                            opcode: CLR,
                            b: b,
                            dest: Some(dest),
                        });
                    }

                    if dest.0 == src.0 && sam == Direct && dam == Direct {
                        return Some(PSEUDO {
                            opcode: NOP,
                            b: b,
                            dest: None,
                        });
                    }
                    if sam == IndirectIncrement && src.0 == SP {
                        if dest.0 == PC {
                            return Some(PSEUDO {
                                opcode: RET,
                                b: b,
                                dest: None,
                            });
                        } else {
                            return Some(PSEUDO {
                                opcode: POP,
                                b: b,
                                dest: Some(dest),
                            });
                        }
                    }
                    if dest.0 == PC && dam == Direct {
                        return Some(PSEUDO {
                            opcode: BR,
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
                                    opcode: CLRC,
                                    b: b,
                                    dest: None,
                                })
                            }
                            2 => {
                                return Some(PSEUDO {
                                    opcode: CLRZ,
                                    b: b,
                                    dest: None,
                                })
                            }
                            4 => {
                                return Some(PSEUDO {
                                    opcode: CLRN,
                                    b: b,
                                    dest: None,
                                })
                            }
                            8 => {
                                return Some(PSEUDO {
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
                                    opcode: SETC,
                                    b: b,
                                    dest: None,
                                })
                            }
                            2 => {
                                return Some(PSEUDO {
                                    opcode: SETZ,
                                    b: b,
                                    dest: None,
                                })
                            }
                            4 => {
                                return Some(PSEUDO {
                                    opcode: SETN,
                                    b: b,
                                    dest: None,
                                })
                            }
                            8 => {
                                return Some(PSEUDO {
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
