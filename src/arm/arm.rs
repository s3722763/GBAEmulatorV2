use quark::BitIndex;
use quark::Signs;

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

enum DataOpcode {
    AND,
    EOR,
    SUB,
    RSB,
    ADD,
    ADC,
    SBC,
    RSC,
    TST,
    TEQ,
    CMP,
    CMN,
    ORR,
    MOV,
    BIC,
    MVN,
}

enum Instruction {
    B {
        cond: Condition,
        link: bool,
        offset: u32,
    },
    BX {
        cond: Condition,
        rn: usize,
    },
    Data_processing {
		cond: Condition,
        imm: bool,
        opcode: DataOpcode,
        s: bool,
        rn: usize,
        rd: usize,
        operand_2: u16,
    },
	MRS {
		cond: Condition,
		source_psr: bool,
		rd: usize
	},
	MSR {
		cond: Condition,
		destination_psr: bool,
		rm: usize
	},
	//Is there a better way to short hand this?
	MSR_Transfer_Imm_PSR_Flag {
		cond: Condition,
		imm: bool,
		destination_psr: bool,
		source_op: u16
	}
}

struct Core {}

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
                unimplemented!("Multiply decode");
            } else if opcode.bits(22..28) == 0x1 && opcode.bits(4..8) == 0x9 {
                unimplemented!("Multiply long decode");
            } else if opcode.bits(22..28) == 0x2 && opcode.bits(4..12) == 0x9 {
                unimplemented!("Single Data Swap decode");
            } else if opcode.bits(4..28) == 0x12FFF1 {
                unimplemented!("Branch and Exchange decode");
            } else if opcode.bits(25..28) == 0x0 && opcode.bit(7) == true && opcode.bit(1) == true {
                if opcode.bit(22) == false && opcode.bits(8..12) == 0 {
                    unimplemented!("Halfword data transfer register offset decode");
                } else {
                    unimplemented!("Halfword data transfer immediate offset decode");
                }
            } else if opcode.bits(26..28) == 1 {
                unimplemented!("Single data transfer decode");
            } else if opcode.bits(25..28) == 0x3 && opcode.bit(4) == true {
                println!("{}: {:X}", "Undefined opcode", opcode);
            } else if opcode.bits(25..28) == 0x4 {
                unimplemented!("Block data transfer decode")
            } else if opcode.bits(25..28) == 0x5 {
                instruction = branch_decode(opcode, cond);
                unimplemented!("Branch decode");
            } else if opcode.bits(25..28) == 0x6 {
                unimplemented!("Coprocessor data transfer decode");
            } else if opcode.bits(25..28) == 0x7 {
                if opcode.bit(4) == true {
                    unimplemented!("Coprocessor Data Operation decode");
                } else {
                    unimplemented!("Coprocessor register transfer decode");
                }
            } else if opcode.bits(24..28) == 0xF {
                unimplemented!("Software interrupt decode")
            } else if opcode.bits(26..28) == 0x0 {
                unimplemented!("Data processing / PSR Tranfer");
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
    let mut offset = opcode.bits(0..24);

    if offset & 0x00800000 != 0 {
        offset |= 0xFF000000;
    };

    let mut instruction = Instruction::B {
        cond,
        link: opcode.bit(24),
        offset,
    };

    return Some(instruction);
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
				source_psr: opcode.bit(22),
				rd: opcode.bits(12..16) as usize
			});
		} else if opcode.bits(4..22) == 0x29F00 {
			//MSR (transfer register contents to PSR)
			return Some(Instruction::MSR {
				cond,
				destination_psr: opcode.bit(22),
				rm: opcode.bits(0..4) as usize
			});
		} else if opcode.bits(23..25) == 0x2 && opcode.bits(12..22) == 0x28F {
			//MSR (transfer register contents or immediate value to PSR flag bits only)
			return Some(Instruction::MSR_Transfer_Imm_PSR_Flag {
				cond,
				imm: opcode.bit(25),
				destination_psr: opcode.bit(22),
				source_op: opcode.bits(0..12) as u16
			});
		}
	}

	if let Some(data_opcode) = get_opcode(opcode.bits(21..25) as u8) {
		let data_instruction = Instruction::Data_processing {
			cond,
			imm: opcode.bit(25),
			opcode: data_opcode,
			s: opcode.bit(20),
			rn: opcode.bits(16..20) as usize,
			rd: opcode.bits(12..16) as usize,
			operand_2: opcode.bits(0..12) as u16
		};

		return Some(data_instruction);
	}

	unreachable!();
}

fn get_opcode(opcode_byte: u8) -> Option<DataOpcode> {
	match opcode_byte {
		0x00 => return Some(DataOpcode::AND),
		0x01 => return Some(DataOpcode::EOR),
		0x02 => return Some(DataOpcode::SUB),
		0x03 => return Some(DataOpcode::RSB),
		0x04 => return Some(DataOpcode::ADD),
		0x05 => return Some(DataOpcode::ADC),
		0x06 => return Some(DataOpcode::SBC),
		0x07 => return Some(DataOpcode::RSC),
		0x08 => return Some(DataOpcode::TST),
		0x09 => return Some(DataOpcode::TEQ),
		0x0A => return Some(DataOpcode::CMP),
		0x0B => return Some(DataOpcode::CMN),
		0x0C => return Some(DataOpcode::ORR),
		0x0D => return Some(DataOpcode::MOV),
		0x0E => return Some(DataOpcode::BIC),
		0x0F => return Some(DataOpcode::MVN),
		_ => None
	}
}