use crate::instructions::{Instruction, Label};

pub struct Runner {
    instructions: Vec<Instruction>,
    registers: [i64; 4096],
    stack: Vec<u64>,
}

impl Runner {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            registers: [0; 4096],
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        let mut pc = 0;

        loop {
            if pc >= self.instructions.len() {
                break;
            }

            let instruction = &self.instructions[pc];

            match instruction {
                Instruction::Set(value, destination) => {
                    let _value = self.read_value(value);
                    let _destination = self.read_address(destination);

                    self.registers[_destination as usize] = _value;
                }
                Instruction::Input(destination) => {
                    let _destination = self.read_address(destination);

                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim().parse::<i64>().unwrap();

                    self.registers[_destination as usize] = input;
                }
                Instruction::Output(value) => {
                    let _value = self.read_value(value);

                    println!("{}", _value);
                }
                Instruction::Add(addend1, addend2, destination) => {
                    let _addend1 = self.read_value(addend1);
                    let _addend2 = self.read_value(addend2);
                    let _destination = self.read_address(destination);

                    self.registers[_destination as usize] = _addend1 + _addend2;
                }
                Instruction::Subtract(minuend, subtrahend, destination) => {
                    let _minuend = self.read_value(minuend);
                    let _subtrahend = self.read_value(subtrahend);
                    let _destination = self.read_address(destination);

                    self.registers[_destination as usize] = _minuend - _subtrahend;
                }
                Instruction::Multiply(factor1, factor2, destination) => {
                    let _factor1 = self.read_value(factor1);
                    let _factor2 = self.read_value(factor2);
                    let _destination = self.read_address(destination);

                    self.registers[_destination as usize] = _factor1 * _factor2;
                }
                Instruction::Divide(dividend, divisor, destination) => {
                    let _dividend = self.read_value(dividend);
                    let _divisor = self.read_value(divisor);
                    let _destination = self.read_address(destination);

                    self.registers[_destination as usize] = _dividend / _divisor;
                }
                Instruction::Modulo(dividend, divisor, destination) => {
                    let _dividend = self.read_value(dividend);
                    let _divisor = self.read_value(divisor);
                    let _destination = self.read_address(destination);

                    self.registers[_destination as usize] = _dividend % _divisor;
                }
                Instruction::Jump(label) => {
                    let _label = match label {
                        Label::Label(_) => panic!("Invalid label"),
                        Label::Instruction(value) => *value,
                    };

                    pc = _label as usize;
                    continue;
                }
                Instruction::JumpGreaterThan(a, b, label) => {
                    let _a = self.read_value(a);
                    let _b = self.read_value(b);
                    let _label = match label {
                        Label::Label(_) => panic!("Invalid label"),
                        Label::Instruction(value) => *value,
                    };

                    if _a > _b {
                        pc = _label as usize;
                        continue;
                    }
                }
                Instruction::JumpEqual(a, b, label) => {
                    let _a = self.read_value(a);
                    let _b = self.read_value(b);
                    let _label = match label {
                        Label::Label(_) => panic!("Invalid label"),
                        Label::Instruction(value) => *value,
                    };

                    if _a == _b {
                        pc = _label as usize;
                        continue;
                    }
                }
                Instruction::JumpLessThan(a, b, label) => {
                    let _a = self.read_value(a);
                    let _b = self.read_value(b);
                    let _label = match label {
                        Label::Label(_) => panic!("Invalid label"),
                        Label::Instruction(value) => *value,
                    };

                    if _a < _b {
                        pc = _label as usize;
                        continue;
                    }
                }
                Instruction::Move(source, destination) => {
                    let _source = self.read_value(source);
                    let _destination = self.read_address(destination);

                    self.registers[_destination as usize] = _source;
                }
                Instruction::Call(label) => {
                    let _label = match label {
                        Label::Label(_) => panic!("Invalid label"),
                        Label::Instruction(value) => *value,
                    };

                    self.stack.push(pc as u64 + 1);
                    pc = _label as usize;
                    continue;
                }
                Instruction::Return => {
                    pc = self.stack.pop().unwrap() as usize;
                    continue;
                }
                Instruction::Time(destination) => {
                    let _destination = self.read_address(destination);

                    let time = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos();

                    self.registers[_destination as usize] = time as i64;
                }
                Instruction::Terminate => return,
            }

            pc += 1;
        }
    }

    fn read_value(&self, param: &crate::instructions::Param) -> i64 {
        match param {
            crate::instructions::Param::Data(value) => *value,
            crate::instructions::Param::Address(value) => self.registers[*value as usize],
            crate::instructions::Param::Reference(value) => {
                let referenced_reg = self.registers[*value as usize];
                self.registers[referenced_reg as usize]
            }
        }
    }

    fn read_address(&self, param: &crate::instructions::Param) -> u64 {
        match param {
            crate::instructions::Param::Address(value) => *value,
            crate::instructions::Param::Reference(value) => self.registers[*value as usize] as u64,
            _ => panic!("Invalid address"),
        }
    }
}