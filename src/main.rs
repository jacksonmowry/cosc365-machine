use std::io::stdin;

fn main() {
    println!("Hello, world!");
}

struct Machine {
    ram: [u8; 4096],
    sp: i16,
    pc: i16,
}

impl Machine {
    pub fn load(&mut self, program: &[u8]) -> Result<(), &'static str> {
        if [0xde, 0xad, 0xbe, 0xef] != program[0..4] {
            // Magic didn't match, bail early
            return Err("Magic didn't match 0xdeadbeef");
        }

        self.ram[0..].clone_from_slice(&program[4..]);
        self.sp = 4095;
        self.pc = 0;

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        // If the instruction does not explicitly move the PC you can just perform the action.
        // If an instruction needs to explicitly move the PC you should:
        // 1. Calculate the new PC
        // 2. Perform any action
        // 3. Set the correct PC value
        // 4. Call `continue` to avoid the 4 byte step at the bottom of the loop
        loop {
            let instruction = self.fetch();

            match instruction {
                Instruction::Exit(_) => break,
                Instruction::Swap(from, to) => {
                    let from_word = u32::from_be_bytes(
                        <[u8; 4]>::try_from(
                            &self.ram[(from << 2) as usize..(from << 2) as usize + 4],
                        )
                        .unwrap(),
                    );
                    let to_word = u32::from_be_bytes(
                        <[u8; 4]>::try_from(&self.ram[(to << 2) as usize..(to << 2) as usize + 4])
                            .unwrap(),
                    );

                    self.ram[(from << 2) as usize..(from << 2) as usize + 4]
                        .copy_from_slice(&to_word.to_be_bytes());
                    self.ram[(to << 2) as usize..(to << 2) as usize + 4]
                        .copy_from_slice(&from_word.to_be_bytes());
                }
                Instruction::Nop() => (),
                Instruction::Input() => {
                    let mut s = String::new();
                    stdin().read_line(&mut s).unwrap();

                    if s.starts_with("0x") || s.starts_with("0X") {
                        // Parse Hex

                    } else if s.starts_with("0b") || s.starts_with("0B") {
                        // Parse Binary
                    } else {
                        // Parse Decimal
                    }
                }
                Instruction::Stinput(_) => todo!(),
                Instruction::Debug(_) => todo!(),
                Instruction::Pop(_) => todo!(),
                Instruction::Add() => todo!(),
                Instruction::Sub() => todo!(),
                Instruction::Mul() => todo!(),
                Instruction::Div() => todo!(),
                Instruction::Rem() => todo!(),
                Instruction::And() => todo!(),
                Instruction::Or() => todo!(),
                Instruction::Xor() => todo!(),
                Instruction::Lsl() => todo!(),
                Instruction::Lsr() => todo!(),
                Instruction::Asr() => todo!(),
                Instruction::Neg() => todo!(),
                Instruction::Not() => todo!(),
                Instruction::Stprint(_) => todo!(),
                Instruction::Call(_) => todo!(),
                Instruction::Return(_) => todo!(),
                Instruction::Goto(_) => todo!(),
                Instruction::IfEq(_) => todo!(),
                Instruction::IfNe(_) => todo!(),
                Instruction::IfLt(_) => todo!(),
                Instruction::IfGt(_) => todo!(),
                Instruction::IfLe(_) => todo!(),
                Instruction::IfGe(_) => todo!(),
                Instruction::EqZero(_) => todo!(),
                Instruction::NeZero(_) => todo!(),
                Instruction::LtZero(_) => todo!(),
                Instruction::GeZero(_) => todo!(),
                Instruction::Dup(_) => todo!(),
                Instruction::Print(_, _) => todo!(),
                Instruction::Dump() => todo!(),
                Instruction::Push(_) => todo!(),
            }

            self.step();
        }

        Ok(())
    }

    fn step(&mut self) {
        self.move_pc(4)
    }

    fn move_pc(&mut self, step: i16) {
        self.pc += step
    }

    // Does not move the program counter, use `step` to move the program counter
    // This is so we don't have to step backwards when using PC-relative offsets
    fn fetch(&self) -> Instruction {
        let instruction_bytes =
            <[u8; 4]>::try_from(&self.ram[self.pc as usize..self.pc as usize + 4]).unwrap();
        let instruction = u32::from_be_bytes(instruction_bytes);
        let opcode = Opcode::from_integer(((instruction >> 28) & 0xf) as u8);

        match opcode {
            Opcode::Miscellaneous => {
                let func4 = (instruction >> 24) & 0xf;

                match func4 {
                    0b0000 => Instruction::Exit(instruction as u8 & 0xf),
                    0b0001 => Instruction::Swap(
                        (instruction >> 12) as i16 & 0xFFF,
                        instruction as i16 & 0xFFF,
                    ),
                    0b0010 => Instruction::Nop(),
                    0b0100 => Instruction::Input(),
                    0b0101 => Instruction::Stinput(instruction & 0xFFFFFF),
                    0b1111 => Instruction::Debug(instruction & 0xFFFFFF),
                    _ => unreachable!("Not a valid func4 for Opcode 0 ({})", func4),
                }
            }
            Opcode::Pop => todo!(),
            Opcode::BinaryArithmetic => todo!(),
            Opcode::UnaryArithmetic => todo!(),
            Opcode::StringPrint => todo!(),
            Opcode::Call => todo!(),
            Opcode::Return => todo!(),
            Opcode::Goto => todo!(),
            Opcode::BinaryIf => todo!(),
            Opcode::UnaryIf => todo!(),
            Opcode::Dup => todo!(),
            Opcode::Print => todo!(),
            Opcode::Dump => todo!(),
            Opcode::Push => todo!(),
        }
    }
}

enum Opcode {
    Miscellaneous = 0b0000,
    Pop = 0b0001,
    BinaryArithmetic = 0b0010,
    UnaryArithmetic = 0b0011,
    StringPrint = 0b0100,
    Call = 0b0101,
    Return = 0b0110,
    Goto = 0b0111,
    BinaryIf = 0b1000,
    UnaryIf = 0b1001,
    Dup = 0b1100,
    Print = 0b1101,
    Dump = 0b1110,
    Push = 0b1111,
}

impl Opcode {
    fn from_integer(val: u8) -> Self {
        match val {
            0 => Self::Miscellaneous,
            1 => Self::Pop,
            2 => Self::BinaryArithmetic,
            3 => Self::UnaryArithmetic,
            4 => Self::StringPrint,
            5 => Self::Call,
            6 => Self::Return,
            7 => Self::Goto,
            8 => Self::BinaryIf,
            9 => Self::UnaryIf,
            12 => Self::Dup,
            13 => Self::Print,
            14 => Self::Dump,
            15 => Self::Push,
            _ => unreachable!(),
        }
    }
}

enum Instruction {
    Exit(u8),
    Swap(i16, i16),
    Nop(),
    Input(),
    Stinput(u32),
    Debug(u32),
    Pop(i32),
    Add(),
    Sub(),
    Mul(),
    Div(),
    Rem(),
    And(),
    Or(),
    Xor(),
    Lsl(),
    Lsr(),
    Asr(),
    Neg(),
    Not(),
    Stprint(i32),
    Call(i32),
    Return(i32),
    Goto(i32),
    IfEq(i32),
    IfNe(i32),
    IfLt(i32),
    IfGt(i32),
    IfLe(i32),
    IfGe(i32),
    EqZero(i32),
    NeZero(i32),
    LtZero(i32),
    GeZero(i32),
    Dup(i32),
    Print(i32, i8),
    Dump(),
    Push(u32),
}
