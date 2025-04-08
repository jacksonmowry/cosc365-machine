use std::io;
use std::io::Write;

fn main() {
    // Just an example of it working for now, this will obv change to accept a real file
    let mut machine = Machine {
        ram: [0; 1024],
        sp: 1024,
        pc: 0,
        input: io::stdin(),
        output: io::stdout(),
    };

    let binary = include_bytes!("../marz/stinput.v");

    // This takes the [u8] that is the file, chunks it into quads,
    // then returns an array of u32 values
    let program: Vec<_> = binary
        .chunks(4)
        .map(|x| u32::from_le_bytes(<[u8; 4]>::try_from(x).unwrap()))
        .collect();

    machine.load(&program).unwrap();
    let exit_code = machine.run().unwrap();

    std::process::exit(exit_code.into());
}

struct Machine<R: io::Read, W: io::Write> {
    ram: [u32; 1024],
    sp: i16,
    pc: i16,
    input: R,
    output: W,
}

impl<R: io::Read, W: io::Write> Machine<R, W> {
    pub fn load(&mut self, program: &[u32]) -> Result<(), &'static str> {
        if 0xefbe_adde != program[0] {
            // Magic didn't match, bail early
            return Err("Magic didn't match 0xdeadbeef");
        }

        self.ram[0..program.len() - 1].clone_from_slice(&program[1..]);
        self.sp = 1024;
        self.pc = 0;

        Ok(())
    }

    pub fn run(&mut self) -> Result<u8, &'static str> {
        // If the instruction does not explicitly move the PC you can just perform the action.
        // If an instruction needs to explicitly move the PC you should:
        // 1. Calculate the new PC
        // 2. Perform any action
        // 3. Set the correct PC value
        // 4. Call `continue` to avoid the 4 byte step at the bottom of the loop
        let exit_code;
        loop {
            let instruction = self.fetch();

            match instruction {
                Instruction::Exit(code) => {
                    exit_code = code;
                    break;
                }
                Instruction::Swap(from, to) => {
                    let tmp = self.ram[(self.sp + (from >> 2)) as usize];
                    self.ram[(self.sp + (from >> 2)) as usize] =
                        self.ram[(self.sp + (to >> 2)) as usize];
                    self.ram[(self.sp + (to >> 2)) as usize] = tmp;
                }
                Instruction::Nop() => (),
                Instruction::Input() => {
                    let s = self.read_line()?;
                    let word: u32;

                    if s.starts_with("0x") || s.starts_with("0X") {
                        // Parse Hex
                        word = u32::from_str_radix(&s.trim()[2..], 16)
                            .expect("Unable to parse hex literal");
                    } else if s.starts_with("0b") || s.starts_with("0B") {
                        // Parse Binary
                        word = u32::from_str_radix(&s.trim()[2..], 2)
                            .expect("Unable to parse binary literal");
                    } else {
                        // Parse Decimal
                        word =
                            u32::from_str_radix(&s, 10).expect("Unable to parse decimal literal");
                    }

                    self.push(word)?;
                }
                Instruction::Stinput(max_chars) => {
                    let mut s = self.read_line()?;
                    s = s.trim().to_string();

                    s.truncate(max_chars as usize);

                    if s.len() % 3 != 0 {
                        let count = 3 - (s.len() % 3);
                        for _i in 0..count {
                            s.push(1 as u8 as char);
                        }
                    }

                    let reversed = s.chars().into_iter().rev().collect::<String>();

                    let push_count = reversed.len() / 3;

                    let s_bytes = reversed.as_bytes();
                    for i in 0..push_count {
                        let mut word: u32 = ((s_bytes[i * 3] as u32) << 16)
                            | ((s_bytes[i * 3 + 1] as u32) << 8)
                            | (s_bytes[i * 3 + 2] as u32);

                        if i != 0 {
                            word |= 0x1 << 24;
                        }

                        self.push(word)?;
                    }
                }
                Instruction::Debug(_) => todo!(),
                Instruction::Pop(offset) => {
                    self.sp += offset as i16 >> 2;
                    if self.sp >= 1024 {
                        self.sp = 1024;
                    }
                }
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
                Instruction::Stprint(offset) => {
                    let mut actual_offset = (self.sp + ((offset as i16) >> 2)) as usize;

                    loop {
                        let bytes = &self.ram[actual_offset].to_be_bytes();
                        if bytes[3] != 1 {
                            self.output.write(&bytes[3..4]).unwrap();
                        }
                        if bytes[2] != 1 {
                            self.output.write(&bytes[2..3]).unwrap();
                        }
                        if bytes[1] != 1 {
                            self.output.write(&bytes[1..2]).unwrap();
                        }

                        if actual_offset == 0 || bytes[0] == 0 {
                            break;
                        }

                        actual_offset += 1;
                    }

                    self.output.flush().unwrap();
                }
                Instruction::Call(_) => todo!(),
                Instruction::Return(_) => todo!(),
                Instruction::Goto(_) => todo!(),
                Instruction::IfEq(_) => todo!(),
                Instruction::IfNe(_) => todo!(),
                Instruction::IfLt(_) => todo!(),
                Instruction::IfGt(_) => todo!(),
                Instruction::IfLe(_) => todo!(),
                Instruction::IfGe(_) => todo!(),
                Instruction::EqZero(offset) => {
                    if self.ram[self.sp as usize] == 0 {
                        self.pc += offset as i16;
                        continue;
                    }
                }
                Instruction::NeZero(_) => todo!(),
                Instruction::LtZero(_) => todo!(),
                Instruction::GeZero(_) => todo!(),
                Instruction::Dup(_) => todo!(),
                Instruction::Print(_, _) => todo!(),
                Instruction::Dump() => todo!(),
                Instruction::Push(val) => self.push(val).unwrap(),
            }

            self.step();
        }

        Ok(exit_code)
    }

    fn step(&mut self) {
        self.move_pc(1)
    }

    fn move_pc(&mut self, step: i16) {
        self.pc += step
    }

    fn push(&mut self, word: u32) -> Result<(), &'static str> {
        if self.sp <= 0 {
            return Err("No room left on stack");
        }

        self.sp -= 1;
        self.ram[self.sp as usize] = word;

        Ok(())
    }

    // Does not move the program counter, use `step` to move the program counter
    // This is so we don't have to step backwards when using PC-relative offsets
    fn fetch(&self) -> Instruction {
        let instruction = self.ram[self.pc as usize];
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
            Opcode::Pop => {
                let offset = instruction & 0xFFFFFFF;

                Instruction::Pop(offset)
            }
            Opcode::BinaryArithmetic => todo!(),
            Opcode::UnaryArithmetic => todo!(),
            Opcode::StringPrint => {
                let offset = instruction & 0xFFFFFFF;

                Instruction::Stprint(offset as i32)
            }
            Opcode::Call => todo!(),
            Opcode::Return => todo!(),
            Opcode::Goto => todo!(),
            Opcode::BinaryIf => todo!(),
            Opcode::UnaryIf => {
                let func2 = (instruction >> 25) & 0b11;
                let mut offset = instruction as i32 & 0xffffff;

                if offset >> 23 == 1 {
                    let mask = 0xff << 24;
                    offset |= mask;
                }

                match func2 {
                    0b00 => Instruction::EqZero(offset),
                    0b01 => Instruction::NeZero(offset),
                    0b10 => Instruction::LtZero(offset),
                    0b11 => Instruction::GeZero(offset),
                    _ => unreachable!("No unary if with this func2"),
                }
            }
            Opcode::Dup => todo!(),
            Opcode::Print => todo!(),
            Opcode::Dump => todo!(),
            Opcode::Push => {
                let mask = 0xF << 28;
                let mut val = instruction & 0xFFFFFFF;

                if val >> 27 == 1 {
                    val |= mask;
                }

                Instruction::Push(val)
            }
        }
    }

    fn read_line(&mut self) -> Result<String, &'static str> {
        let mut s = String::new();
        let mut buf = [0; 1];

        loop {
            let read = self.input.read(&mut buf[..]).unwrap();
            if read == 0 {
                break;
            }

            if buf[0] as char == '\n' || buf[0] as char == '\0' {
                break;
            }

            s.push(buf[0] as char);
        }

        Ok(s)
    }
}

#[derive(Debug)]
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
            _ => unreachable!("I got {} which is not a valid opcode", val),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Exit(u8),
    Swap(i16, i16),
    Nop(),
    Input(),
    Stinput(u32),
    Debug(u32),
    Pop(u32),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_machine() {
        let mut machine = Machine {
            ram: [0; 1024],
            sp: 1024,
            pc: 0,
            input: io::Cursor::new(Vec::new()),
            output: io::Cursor::new(Vec::new()),
        };

        let program = &[0xefbe_adde, 0x0005_0000, 0x0000_0000];

        machine.load(program).unwrap();

        assert_eq!(machine.ram[..program.len() - 1], program[1..]);
    }

    #[test]
    fn test_input() {
        let mut machine = Machine {
            ram: [0; 1024],
            sp: 1024,
            pc: 0,
            input: io::Cursor::new(Vec::new()),
            output: io::Cursor::new(Vec::new()),
        };

        let program = &[0xefbe_adde, 0x0400_0000];

        let input: u32 = 0x45;
        machine
            .input
            .get_mut()
            .write(format!("{:#x}", input).as_bytes())
            .unwrap();

        machine.load(program).unwrap();
        machine.run().unwrap();

        let word = machine.ram[machine.sp as usize];
        assert_eq!(word, input)
    }

    #[test]
    fn test_stinput() {
        let mut machine = Machine {
            ram: [0; 1024],
            sp: 1024,
            pc: 0,
            input: io::Cursor::new(Vec::new()),
            output: io::Cursor::new(Vec::new()),
        };

        let program = &[0xefbe_adde, 0x0500_00FF];

        machine
            .input
            .get_mut()
            .write(format!("Hello World\n").as_bytes()) // This whitespace will be trimmed
            .unwrap();

        machine.load(program).unwrap();
        machine.run().unwrap();

        let words = &machine.ram[machine.sp as usize..machine.sp as usize + 4];

        // This weird array is the string in reverse order, grouped into triplets,
        // and padded with 0/1 depending on if we're at the end of the string
        assert_eq!(
            &[0x016c_6548, 0x0120_6f6c, 0x0172_6f57, 0x0001_646c],
            &words
        );
        assert_eq!(1024 - 4, machine.sp);
    }

    #[test]
    fn test_stprint() {
        let mut machine = Machine {
            ram: [0; 1024],
            sp: 1024,
            pc: 0,
            input: io::Cursor::new(Vec::new()),
            output: io::Cursor::new(Vec::new()),
        };

        let program = &[0xefbe_adde, 0x0500_00FF, 0x4000_0000];

        machine
            .input
            .get_mut()
            .write(format!("Hello World!").as_bytes())
            .unwrap();

        machine.load(program).unwrap();
        machine.run().unwrap();

        let output = machine.output.clone().into_inner();
        let output_str = String::from_utf8(output).unwrap();

        assert_eq!("Hello World!", output_str);
    }

    #[test]
    fn test_push() {
        let mut machine = Machine {
            ram: [0; 1024],
            sp: 1024,
            pc: 0,
            input: io::Cursor::new(Vec::new()),
            output: io::Cursor::new(Vec::new()),
        };

        let program = &[0xefbe_adde, 0xf000_0045];

        machine.load(program).unwrap();
        machine.run().unwrap();

        let word = machine.ram[machine.sp as usize];

        assert_eq!(0x45, word);
        assert_eq!(1023, machine.sp);
    }

    #[test]
    fn test_push_negative() {
        let mut machine = Machine {
            ram: [0; 1024],
            sp: 1024,
            pc: 0,
            input: io::Cursor::new(Vec::new()),
            output: io::Cursor::new(Vec::new()),
        };

        // Push -4
        let program = &[0xefbe_adde, 0xffff_fffc];

        machine.load(program).unwrap();
        machine.run().unwrap();

        let word = i32::from_ne_bytes(machine.ram[machine.sp as usize].to_ne_bytes());

        assert_eq!(-4, word);
        assert_eq!(1023, machine.sp);
    }

    #[test]
    fn test_pop() {
        let mut machine = Machine {
            ram: [0; 1024],
            sp: 1024,
            pc: 0,
            input: io::Cursor::new(Vec::new()),
            output: io::Cursor::new(Vec::new()),
        };

        let program = &[0xefbe_adde, 0xf000_0045, 0x1000_0004];

        machine.load(program).unwrap();
        machine.run().unwrap();

        assert_eq!(1024, machine.sp);
    }

    #[test]
    fn test_stinput_marz() {
        let mut machine = Machine {
            ram: [0; 1024],
            sp: 1024,
            pc: 0,
            input: io::Cursor::new(Vec::new()),
            output: io::Cursor::new(Vec::new()),
        };

        let binary = include_bytes!("../marz/stinput.v");

        // This takes the [u8] that is the file, chunks it into quads,
        // then returns an array of u32 values
        let program: Vec<_> = binary
            .chunks(4)
            .map(|x| u32::from_le_bytes(<[u8; 4]>::try_from(x).unwrap()))
            .collect();

        machine
            .input
            .get_mut()
            .write(format!("Hii\n").as_bytes())
            .unwrap();

        machine.load(&program).unwrap();
        machine.run().unwrap();

        let output = machine.output.clone().into_inner();
        let output_str = String::from_utf8(output).unwrap();

        assert_eq!("Enter a string: You wrote = 'Hii'\n", output_str);
    }
}
