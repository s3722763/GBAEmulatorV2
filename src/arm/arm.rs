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
	Always
}

enum Instruction {
	B { cond: Condition, link: bool, offset: u32 }
}

struct Core {

}

impl Condition {
	pub fn new(code: u8) -> Option<Self> {
		match code {
			0x0 => {
				Some(Condition::Equal)
			},
			0x1 => {
				Some(Condition::NotEqual)
			},
			0x2 => {
				Some(Condition::UnsignedHigherOrSame)
			},
			0x3 => {
				Some(Condition::UnsignedLower)
			},
			0x4 => {
				Some(Condition::Negative)
			},
			0x5 => {
				Some(Condition::PositiveOrZero)
			},
			0x6 => {
				Some(Condition::Overflow)
			},
			0x7 => {
				Some(Condition::NoOverflow)
			},
			0x8 => {
				Some(Condition::UnsignedHigher)
			},
			0x9 => {
				Some(Condition::UnsignedLowerOrSame)
			},
			0xA => {
				Some(Condition::GreaterOrEqual)
			},
			0xB => {
				Some(Condition::LessThan)
			},
			0xC => {
				Some(Condition::GreaterThan)
			},
			0xD => {
				Some(Condition::LessThanOfEqual)
			},
			0xE => {
				Some(Condition::Always)
			},
			_ => {
				unreachable!("Invalid condition code {}", code);
			}
		}
	}
}

impl Core {
	pub fn cycle() {

	}

	fn fetch(&self) {

	}

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
			} else {
				unreachable!();
			}
		}

		return instruction;
	}

	fn execute(&self, instruction: Instruction) {

	}

	fn write_back(&self) {

	}
}

fn branch_decode(opcode: u32, cond: Condition) -> Option<Instruction> {
	if opcode.bits(25..28) == 0x5 {
		let mut offset =opcode.bits(0..24);

		if offset & 0x00800000 != 0 {
			offset |= 0xFF000000;
		};

		let mut instruction = Instruction::B {
			cond,
			link: opcode.bit(24),
			offset
		};

		return Some(instruction);
	}

	None
}