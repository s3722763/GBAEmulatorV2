use quark::BitIndex;

enum Instruction {

}

struct Core {

}

impl Core {
	pub fn cycle() {

	}

	fn fetch(&self) {

	}

	fn decode(&self, opcode: u32) {
		if opcode.bits(22..28) == 0x0  && opcode.bits(4..8) == 0x9 {
			unimplemented!("Multiply decode");
		} else if opcode.bits(22..28) == 0x1 && opcode.bits(4..8) == 0x9 {
			unimplemented!("Multiply long decode");
		} else if opcode.bits(22..28) == 0x2 && opcode.bits(4..12) == 0x9 {
			unimplemented!("Single Data Swap decode");
		} else if opcode.bits(4..28) == 0x12FFF1 {
			unimplemented!("Branch and Exchange decode");
		} else if opcode.bits(25..28) == 0x0 && opcode.bit(7) == true && opcode.bit(1) == true {
			if opcode.bit(22) == false && opcode.bits(8..12) == 0 {

			} else {

			}
		} else if opcode.bits(26..28) == 1 {
			unimplemented!("Single data transfer decode");
		} else if opcode.bits(25..28) == 0x3 && opcode.bit(4) == true {
			println!("{}: {:X}", "Undefined opcode", opcode);
		} else if opcode.bits(25..28) == 0x4 {
			unimplemented!("Block data transfer decode")
		} else if opcode.bits(25..28) == 0x5 {
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

	fn execute(&self, instruction: Instruction) {

	}

	fn write_back() {

	}
}