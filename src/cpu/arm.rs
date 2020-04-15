use quark::BitIndex;
use quark::Signs;
use std::fmt::Display;
use failure::_core::fmt::{Formatter, Error};

enum Condition {
    Equal,
    NotEqual,
    UnsignedHigherOrSame,
    UnsignedLower,
    Negative,
    PositiveOrZero,
    Overflow,
    NoOverflow,
    UnsignedHigher,
    UnsignedLowerOrSame,
    GreaterOrEqual,
    LessThan,
    GreaterThan,
    LessThanOfEqual,
    Always,
}

enum Instruction {
	//Branch
	B { cond: Condition, offset: usize },
	BL { cond: Condition, offset: usize },
	BX { cond: Condition, rn: usize},
	//Data Processing
	AND { cond: Condition, imm: bool, s: bool, rn: usize, rd: usize, operand: u16 },
	EOR { cond: Condition, imm: bool, s: bool, rn: usize, rd: usize, operand: u16 },
	SUB { cond: Condition, imm: bool, s: bool, rn: usize, rd: usize, operand: u16 },
	RSB { cond: Condition, imm: bool, s: bool, rn: usize, rd: usize, operand: u16 },
	ADD { cond: Condition, imm: bool, s: bool, rn: usize, rd: usize, operand: u16 },
	ADC { cond: Condition, imm: bool, s: bool, rn: usize, rd: usize, operand: u16 },
	SBC { cond: Condition, imm: bool, s: bool, rn: usize, rd: usize, operand: u16 },
	RSC { cond: Condition, imm: bool, s: bool, rn: usize, rd: usize, operand: u16 },
	TST { cond: Condition, imm: bool, s: bool, rn: usize, rd: usize, operand: u16 },
	TEQ { cond: Condition, imm: bool, s: bool, rn: usize, rd: usize, operand: u16 },
	CMP { cond: Condition, imm: bool, s: bool, rn: usize, rd: usize, operand: u16 },
	CMN { cond: Condition, imm: bool, s: bool, rn: usize, rd: usize, operand: u16 },
	ORR { cond: Condition, imm: bool, s: bool, rn: usize, rd: usize, operand: u16 },
	MOV { cond: Condition, imm: bool, s: bool, rn: usize, rd: usize, operand: u16 },
	BIC { cond: Condition, imm: bool, s: bool, rn: usize, rd: usize, operand: u16 },
	MVN { cond: Condition, imm: bool, s: bool, rn: usize, rd: usize, operand: u16 },
	//PSR Transfer
	MRS { cond: Condition, Ps: bool, rd: usize },
	MSR { cond: Condition, Pd: bool, I: Option<bool>, rm: Option<usize>, source: Option<u16> },
	//Multiply
	MLA { cond: Condition, S: bool, rd: usize, rn: usize, rs: usize, rm: usize },
	MLAL { cond: Condition, U: bool, S: bool, rdhi: usize, rdlo: usize, rs: usize, rm: usize },
	MUL { cond: Condition, S: bool, rd: usize, rn: usize, rs: usize, rm: usize },
	MULL { cond: Condition, U: bool, S: bool, rdhi: usize, rdlo: usize, rs: usize, rm: usize },
	//Single data transfer
	LDR { cond: Condition, I: bool, P: bool, U: bool, B: bool, W: bool, rn: usize, rd: usize, offset: u16},
	STR { cond: Condition, I: bool, P: bool, U: bool, B: bool, W: bool, rn: usize, rd: usize, offset: u16},
	//Halfword and Signed data transfer
	LDRH { cond: Condition, P: bool, U: bool, W: bool, rn: usize, rd: usize, rm: usize, is_offset: bool },
	LDRSB { cond: Condition, P: bool, U: bool, W: bool, rn: usize, rd: usize, rm: usize, is_offset: bool },
	LDRSH { cond: Condition, P: bool, U: bool, W: bool, rn: usize, rd: usize, rm: usize, is_offset: bool },
	SDRH { cond: Condition, P: bool, U: bool, W: bool, rn: usize, rd: usize, rm: usize, is_offset: bool },
	SDRSB { cond: Condition, P: bool, U: bool, W: bool, rn: usize, rd: usize, rm: usize, is_offset: bool },
	SDRSH { cond: Condition, P: bool, U: bool, W: bool, rn: usize, rd: usize, rm: usize, is_offset: bool },
	//Block data transfer
	LDM { cond: Condition, P: bool, U: bool, S: bool, W: bool, rn: usize, register_list: u16 },
	STM { cond: Condition, P: bool, U: bool, S: bool, W: bool, rn: usize, register_list: u16 },
	//Single data swap
	SWP { cond: Condition, B: bool, rn: usize, rd: usize, rm: usize },
	//Software interrupt
	SWI { cond: Condition },
	//Coprocessor Data Operations
	CDP { cond: Condition, cp_opc: usize, c_rn: usize, c_rd: usize, cp_no: usize, cp: usize, c_rm: usize },
	//Coprocessor Data Transfers
	LDC { cond: Condition, P: bool, U: bool, N: bool, W: bool, rn: usize, c_rd: usize, cp_no: usize, offset: u8},
	STC { cond: Condition, P: bool, U: bool, N: bool, W: bool, rn: usize, c_rd: usize, cp_no: usize, offset: u8},
	//Coprocessor Register Transfers
	MCR { cond: Condition, cp_opc: usize, c_rn: usize, rd: usize, cp_no: usize, cp: usize, c_rm: usize},
	MRC { cond: Condition, cp_opc: usize, c_rn: usize, rd: usize, cp_no: usize, cp: usize, c_rm: usize},
}

struct Core {}

impl Display for Instruction {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
		unimplemented!()
	}
}

impl Condition {
    pub fn new(code: u8) -> Option<Self> {
        match code {
            0x0 => Some(Condition::Equal),
            0x1 => Some(Condition::NotEqual),
            0x2 => Some(Condition::UnsignedHigherOrSame),
            0x3 => Some(Condition::UnsignedLower),
            0x4 => Some(Condition::Negative),
            0x5 => Some(Condition::PositiveOrZero),
            0x6 => Some(Condition::Overflow),
            0x7 => Some(Condition::NoOverflow),
            0x8 => Some(Condition::UnsignedHigher),
            0x9 => Some(Condition::UnsignedLowerOrSame),
            0xA => Some(Condition::GreaterOrEqual),
            0xB => Some(Condition::LessThan),
            0xC => Some(Condition::GreaterThan),
            0xD => Some(Condition::LessThanOfEqual),
            0xE => Some(Condition::Always),
            _ => {
                unreachable!("Invalid condition code {}", code);
            }
        }
    }
}

impl Core {
    pub fn cycle() {}

    fn fetch(&self) {}

    fn decode(&self, opcode: u32) -> Option<Instruction> {
        let mut instruction = None;

        if let Some(cond) = Condition::new(opcode.bits(28..32) as u8) {
            if opcode.bits(22..28) == 0x0 && opcode.bits(4..8) == 0x9 {
                instruction = multiply_decode(opcode, cond);
            } else if opcode.bits(22..28) == 0x1 && opcode.bits(4..8) == 0x9 {
				instruction = multiply_long_decode(opcode, cond);
            } else if opcode.bits(22..28) == 0x2 && opcode.bits(4..12) == 0x9 {
				instruction = single_data_transfer_decode(opcode, cond);
            } else if opcode.bits(4..28) == 0x12FFF1 {
                instruction =  branch_exchange_decode(opcode, cond);
            } else if opcode.bits(25..28) == 0x0 && opcode.bit(7) == true && opcode.bit(1) == true {
				//TODO: Check if this works
                if opcode.bit(22) == false && opcode.bits(8..12) == 0 {
					//Not sure if this will work
                    instruction = halfword_data_transfer_decode(opcode, cond);
                } else {
					//Not sure if this will work
					instruction = halfword_data_transfer_decode(opcode, cond);
                }
            } else if opcode.bits(26..28) == 1 {
				instruction = single_data_transfer_decode(opcode, cond);
            } else if opcode.bits(25..28) == 0x3 && opcode.bit(4) == true {
                println!("{}: {:X}", "Undefined opcode", opcode);
            } else if opcode.bits(25..28) == 0x4 {
				block_data_transfer_decode(opcode, cond);
            } else if opcode.bits(25..28) == 0x5 {
                instruction = branch_decode(opcode, cond);
            } else if opcode.bits(25..28) == 0x6 {
                instruction = coprocessor_data_transfer_decode(opcode, cond);
            } else if opcode.bits(25..28) == 0x7 {
                if opcode.bit(4) == true {
                    instruction = coprocessor_data_operation_decode(opcode, cond);
                } else {
                    instruction = coprocessor_register_transfer_decode(opcode, cond);
                }
            } else if opcode.bits(24..28) == 0xF {
                instruction = software_interrupt_decode(opcode, cond);
            } else if opcode.bits(26..28) == 0x0 {
				instruction = data_processing_decode(opcode, cond);
            } else {
                unreachable!();
            }
        }

        return instruction;
    }

    fn execute(&self, instruction: Instruction) {}

    fn write_back(&self) {}
}

fn branch_decode(opcode: u32, cond: Condition) -> Option<Instruction> {
	let mut instruction = None;
	let mut offset = opcode.bits(0..24) as usize;

    if offset & 0x00800000 != 0 {
        offset |= 0xFF000000;
    };

	if opcode.bit(24) {
		instruction = Some(Instruction::BL {
			cond,
			offset,
		});
	} else {
		instruction = Some(Instruction::B {
			cond,
			offset,
		});
	}
    return instruction;
}

fn branch_exchange_decode(opcode: u32, cond: Condition) -> Option<Instruction> {
    let opcode = Instruction::BX {
        cond,
        rn: opcode.bits(0..4) as usize,
    };

    return Some(opcode);
}

fn data_processing_decode(opcode: u32, cond: Condition) -> Option<Instruction> {
	if opcode.bits(23..28) == 0x2 {
		if opcode.bits(16..22) == 0xF && opcode.bits(0..12) == 0x0 {
			//MRS (transfer PSR contents to a register)
			return Some(Instruction::MRS {
				cond,
				Ps: opcode.bit(22),
				rd: opcode.bits(12..16) as usize,

			});
		} else if opcode.bits(4..22) == 0x29F00 {
			//MSR (transfer register contents to PSR)
			return Some(Instruction::MSR {
				cond,
				Pd: opcode.bit(22),
				I: None,
				rm: Some(opcode.bits(0..4) as usize),
				source: None
			});
		} else if opcode.bits(23..25) == 0x2 && opcode.bits(12..22) == 0x28F {
			//MSR (transfer register contents or immediate value to PSR flag bits only)
			return Some(Instruction::MSR {
				cond,
				Pd: opcode.bit(22),
				I: Some(opcode.bit(25)),
				source: Some(opcode.bits(0..12) as u16),
				rm: None
			});
		}
	}

	unreachable!();
}

fn multiply_decode(opcode: u32, cond: Condition) -> Option<Instruction> {
	return if opcode.bit(21) {
		Some(Instruction::MLA {
			cond,
			S: opcode.bit(20),
			rd: opcode.bits(16..20) as usize,
			rn: opcode.bits(12..16) as usize,
			rs: opcode.bits(8..12) as usize,
			rm: opcode.bits(0..4) as usize
		})
	} else {
		Some(Instruction::MUL {
			cond,
			S: opcode.bit(20),
			rd: opcode.bits(16..20) as usize,
			rn: opcode.bits(12..16) as usize,
			rs: opcode.bits(8..12) as usize,
			rm: opcode.bits(0..4) as usize
		})
	}
}

fn multiply_long_decode(opcode: u32, cond: Condition) -> Option<Instruction> {
	return if opcode.bit(21) {
		Some(Instruction::MULL {
			cond,
			U: opcode.bit(22),
			S: opcode.bit(20),
			rdhi: opcode.bits(16..20) as usize,
			rdlo: opcode.bits(12..16) as usize,
			rs: opcode.bits(8..12) as usize,
			rm: opcode.bits(0..4) as usize
		})
	} else {
		Some(Instruction::MLAL {
			cond,
			U: opcode.bit(22),
			S: opcode.bit(20),
			rdhi: opcode.bits(16..20) as usize,
			rdlo: opcode.bits(12..16) as usize,
			rs: opcode.bits(8..12) as usize,
			rm: opcode.bits(0..4) as usize
		})
	}
}

fn single_data_transfer_decode(opcode: u32, cond: Condition) -> Option<Instruction> {
	return if opcode.bit(20) {
		Some(Instruction::LDR {
			cond,
			I: opcode.bit(25),
			P: opcode.bit(24),
			U: opcode.bit(23),
			B: opcode.bit(22),
			W: opcode.bit(21),
			rn: opcode.bits(16..20) as usize,
			rd: opcode.bits(12..26) as usize,
			offset: opcode.bits(0..12) as u16
		})
	} else {
		Some(Instruction::STR {
			cond,
			I: opcode.bit(25),
			P: opcode.bit(24),
			U: opcode.bit(23),
			B: opcode.bit(22),
			W: opcode.bit(21),
			rn: opcode.bits(16..20) as usize,
			rd: opcode.bits(12..26) as usize,
			offset: opcode.bits(0..12) as u16
		})
	}
}

fn halfword_data_transfer_decode(opcode: u32, cond: Condition) -> Option<Instruction> {
	//S - L
	//H - SB - SH
	return if opcode.bit(20) {
		//Load
		if opcode.bit(6) {
			//Signed data
			if opcode.bit(5) {
				//Signed halfword
				Some(Instruction::LDRSH {
					cond,
					P: opcode.bit(24),
					U: opcode.bit(23),
					W: opcode.bit(21),
					rn: opcode.bits(16..20) as usize,
					rd: opcode.bits(12..16) as usize,
					rm: opcode.bits(0..4) as usize,
					is_offset: opcode.bit(22)
				})
			} else {
				//Signed byte
				Some(Instruction::LDRSB {
					cond,
					P: opcode.bit(24),
					U: opcode.bit(23),
					W: opcode.bit(21),
					rn: opcode.bits(16..20) as usize,
					rd: opcode.bits(12..16) as usize,
					rm: opcode.bits(0..4) as usize,
					is_offset: opcode.bit(22)
				})
			}
		} else {
			Some(Instruction::LDRH {
				cond,
				P: opcode.bit(24),
				U: opcode.bit(23),
				W: opcode.bit(21),
				rn: opcode.bits(16..20) as usize,
				rd: opcode.bits(12..16) as usize,
				rm: opcode.bits(0..4) as usize,
				is_offset: opcode.bit(22)
			})
		}
	} else {
		//Store
		if opcode.bit(6) {
			//Signed data
			if opcode.bit(5) {
				//Signed halfword
				Some(Instruction::SDRSH {
					cond,
					P: opcode.bit(24),
					U: opcode.bit(23),
					W: opcode.bit(21),
					rn: opcode.bits(16..20) as usize,
					rd: opcode.bits(12..16) as usize,
					rm: opcode.bits(0..4) as usize,
					is_offset: opcode.bit(22)
				})
			} else {
				//Signed byte
				Some(Instruction::SDRSB {
					cond,
					P: opcode.bit(24),
					U: opcode.bit(23),
					W: opcode.bit(21),
					rn: opcode.bits(16..20) as usize,
					rd: opcode.bits(12..16) as usize,
					rm: opcode.bits(0..4) as usize,
					is_offset: opcode.bit(22)
				})
			}
		} else {
			Some(Instruction::SDRH {
				cond,
				P: opcode.bit(24),
				U: opcode.bit(23),
				W: opcode.bit(21),
				rn: opcode.bits(16..20) as usize,
				rd: opcode.bits(12..16) as usize,
				rm: opcode.bits(0..4) as usize,
				is_offset: opcode.bit(22)
			})
		}
	}
}

fn block_data_transfer_decode(opcode: u32, cond: Condition) -> Option<Instruction> {
	return if opcode.bit(20) == true {
		//LDM
		Some(Instruction::LDM {
			cond,
			P: opcode.bit(24),
			U: opcode.bit(23),
			S: opcode.bit(22),
			W: opcode.bit(21),
			rn: opcode.bits(16..20) as usize,
			register_list: opcode.bits(0..16) as u16
		})
	} else {
		//STM
		Some(Instruction::STM {
			cond,
			P: opcode.bit(24),
			U: opcode.bit(23),
			S: opcode.bit(22),
			W: opcode.bit(21),
			rn: opcode.bits(16..20) as usize,
			register_list: opcode.bits(0..16) as u16
		})
	}
}

fn coprocessor_data_transfer_decode(opcode: u32, cond: Condition) -> Option<Instruction> {
	return if opcode.bit(20) == true {
		Some(Instruction::LDC {
			cond,
			P: opcode.bit(24),
			U: opcode.bit(23),
			N: opcode.bit(22),
			W: opcode.bit(21),
			rn: opcode.bits(16..20) as usize,
			c_rd: opcode.bits(12..16) as usize,
			cp_no: opcode.bits(8..12) as usize,
			offset: opcode.bits(0..8) as u8
		})
	} else {
		Some(Instruction::STC {
			cond,
			P: opcode.bit(24),
			U: opcode.bit(23),
			N: opcode.bit(22),
			W: opcode.bit(21),
			rn: opcode.bits(16..20) as usize,
			c_rd: opcode.bits(12..16) as usize,
			cp_no: opcode.bits(8..12) as usize,
			offset: opcode.bits(0..8) as u8
		})
	}
}

fn coprocessor_register_transfer_decode(opcode: u32, cond: Condition) -> Option<Instruction> {
	return if opcode.bit(20) == true {
		Some(Instruction::MRC {
			cond,
			cp_opc: opcode.bits(21..24) as usize,
			c_rn: opcode.bits(16..20) as usize,
			rd: opcode.bits(12..16) as usize,
			cp_no: opcode.bits(8..12) as usize,
			cp: opcode.bits(5..8) as usize,
			c_rm: opcode.bits(0..4) as usize
		})
	} else {
		Some(Instruction::MCR {
			cond,
			cp_opc: opcode.bits(21..24) as usize,
			c_rn: opcode.bits(16..20) as usize,
			rd: opcode.bits(12..16) as usize,
			cp_no: opcode.bits(8..12) as usize,
			cp: opcode.bits(5..8) as usize,
			c_rm: opcode.bits(0..4) as usize
		})
	}
}

fn coprocessor_data_operation_decode(opcode: u32, cond: Condition) -> Option<Instruction> {
	return Some(Instruction::CDP {
		cond,
		cp_opc: opcode.bits(20..24) as usize,
		c_rn: opcode.bits(16..20) as usize,
		c_rd: opcode.bits(12..16) as usize,
		cp_no: opcode.bits(8..12) as usize,
		cp: opcode.bits(5..8) as usize,
		c_rm: opcode.bits(0..4) as usize
	})
}

fn software_interrupt_decode(opcode: u32, cond: Condition) -> Option<Instruction> {
	return Some(Instruction::SWI {
		cond
	})
}