use std::io;
use std::io::Write;

enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    fn parse(code: isize) -> Self {
        match code {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            x => panic!("Unknown parameter code {}", x),
        }
    }
}

enum Opcode {
    Add,
    Mul,
    Read,
    Write,
    Halt,
}

impl Opcode {
    fn parse(code: isize) -> Self {
        match code {
            1 => Opcode::Add,
            2 => Opcode::Mul,
            3 => Opcode::Read,
            4 => Opcode::Write,
            99 => Opcode::Halt,
            x => panic!("Unknown opcode {}", x),
        }
    }
}

type Instruction = (Opcode, ParameterMode, ParameterMode, ParameterMode);

pub struct CPU {
    memory: Vec<isize>,
    ip: usize,
    halted: bool,
}

impl CPU {
    pub fn new(program: Vec<isize>) -> Self {
        CPU {
            memory: program,
            ip: 0,
            halted: false,
        }
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.step();
        }
    }

    fn parse_instruction(code: isize) -> Instruction {
        let opcode = code % 100;
        let opcode = Opcode::parse(opcode);

        let pmode_1 = (code / 100) % 10;
        let pmode_1 = ParameterMode::parse(pmode_1);

        let pmode_2 = (code / 1000) % 10;
        let pmode_2 = ParameterMode::parse(pmode_2);

        let pmode_3 = (code / 10000) % 10;
        let pmode_3 = ParameterMode::parse(pmode_3);

        (opcode, pmode_1, pmode_2, pmode_3)
    }

    fn get_operand_addr(&self, addr: usize, mode: ParameterMode) -> usize {
        match mode {
            ParameterMode::Immediate => addr,
            ParameterMode::Position => self.memory[addr] as usize,
        }
    }

    fn step(&mut self) {
        let (opcode, pmode_1, pmode_2, pmode_3) = Self::parse_instruction(self.memory[self.ip]);
        match opcode {
            Opcode::Add => self.opcode_add((pmode_1, pmode_2, pmode_3)),
            Opcode::Mul => self.opcode_mul((pmode_1, pmode_2, pmode_3)),
            Opcode::Read => self.opcode_read(pmode_1),
            Opcode::Write => self.opcode_write(pmode_1),
            Opcode::Halt => self.step_halt(),
        }
    }

    fn opcode_add(&mut self, pmode: (ParameterMode, ParameterMode, ParameterMode)) {
        let operand1 = self.memory[self.get_operand_addr(self.ip + 1, pmode.0)];
        let operand2 = self.memory[self.get_operand_addr(self.ip + 2, pmode.1)];

        let target_pos = self.get_operand_addr(self.ip + 3, pmode.2);
        self.memory[target_pos] = operand1 + operand2;

        self.ip += 4;
    }

    fn opcode_mul(&mut self, pmode: (ParameterMode, ParameterMode, ParameterMode)) {
        let operand1 = self.memory[self.get_operand_addr(self.ip + 1, pmode.0)];
        let operand2 = self.memory[self.get_operand_addr(self.ip + 2, pmode.1)];

        let target_pos = self.get_operand_addr(self.ip + 3, pmode.2);
        self.memory[target_pos] = operand1 * operand2;

        self.ip += 4;
    }

    fn opcode_read(&mut self, pmode: ParameterMode) {
        let target_pos = self.get_operand_addr(self.ip + 1, pmode);

        let mut buffer = String::new();
        print!("?> ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout in opcode_read");
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read buffer from stdin");
        let integer = buffer
            .trim()
            .parse::<isize>()
            .expect("Failed to parse integer from stdin");

        self.memory[target_pos] = integer;
        self.ip += 2;
    }

    fn opcode_write(&mut self, pmode: ParameterMode) {
        let operand = self.memory[self.get_operand_addr(self.ip + 1, pmode)];

        println!("#> {}", operand);
        self.ip += 2;
    }

    fn step_halt(&mut self) {
        self.halted = true;
    }
}
