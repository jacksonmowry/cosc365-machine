fn main() {
    println!("Hello, world!");
}

struct Machine {
    ram: [u8; 4096],
    sp: usize,
    pc: usize,
}

impl Machine {
    // Does not move the program counter, use `step` to move the program counter
    // This is so we don't have to step backwards when using PC-relative offsets
    pub fn fetch(&self) -> Instruction {
        let instruction_bytes = <[u8; 4]>::try_from(&self.ram[self.pc..self.pc + 4]).unwrap();
        let instruction = u32::from_be_bytes(instruction_bytes);
        let opcode = Opcode::from_integer(((instruction >> 28) & 0xf) as u8);

        match opcode {
            Opcode::Miscellaneous => {}
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

        return Instruction::Nop();
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
