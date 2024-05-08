use crate::instructions::{Instruction, Label};
use crate::syscall::syscall;

pub struct Runner {
    instructions: Vec<Instruction>,
    registers: [i64; 4096],
    stack: Vec<usize>,
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
        let max_pc = self.instructions.len();

        loop {
            if pc >= max_pc {
                break;
            }

            let instruction = &self.instructions[pc];

            match instruction {
                Instruction::Set(value, destination) => {
                    let _value = self.read_source(value);
                    let _destination = self.read_destination(destination);

                    self.registers[_destination] = _value;
                }
                Instruction::Input(destination) => {
                    let _destination = self.read_destination(destination);

                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim().parse::<i64>().unwrap();

                    self.registers[_destination] = input;
                }
                Instruction::Output(value) => {
                    let _value = self.read_source(value);

                    println!("{}", _value);
                }
                Instruction::Add(addend1, addend2, destination) => {
                    let _addend1 = self.read_source(addend1);
                    let _addend2 = self.read_source(addend2);
                    let _destination = self.read_destination(destination);

                    self.registers[_destination] = _addend1 + _addend2;
                }
                Instruction::Subtract(minuend, subtrahend, destination) => {
                    let _minuend = self.read_source(minuend);
                    let _subtrahend = self.read_source(subtrahend);
                    let _destination = self.read_destination(destination);

                    self.registers[_destination] = _minuend - _subtrahend;
                }
                Instruction::Multiply(factor1, factor2, destination) => {
                    let _factor1 = self.read_source(factor1);
                    let _factor2 = self.read_source(factor2);
                    let _destination = self.read_destination(destination);

                    self.registers[_destination] = _factor1 * _factor2;
                }
                Instruction::Divide(dividend, divisor, destination) => {
                    let _dividend = self.read_source(dividend);
                    let _divisor = self.read_source(divisor);
                    let _destination = self.read_destination(destination);

                    self.registers[_destination] = _dividend / _divisor;
                }
                Instruction::Modulo(dividend, divisor, destination) => {
                    let _dividend = self.read_source(dividend);
                    let _divisor = self.read_source(divisor);
                    let _destination = self.read_destination(destination);

                    self.registers[_destination] = _dividend % _divisor;
                }
                Instruction::Jump(label) => {
                    let _label = match label {
                        Label::Label(_) => panic!("Invalid label"),
                        Label::Instruction(value) => *value,
                    };

                    pc = _label;
                    continue;
                }
                Instruction::JumpGreaterThan(a, b, label) => {
                    let _a = self.read_source(a);
                    let _b = self.read_source(b);
                    let _label = match label {
                        Label::Label(_) => panic!("Invalid label"),
                        Label::Instruction(value) => *value,
                    };

                    if _a > _b {
                        pc = _label;
                        continue;
                    }
                }
                Instruction::JumpEqual(a, b, label) => {
                    let _a = self.read_source(a);
                    let _b = self.read_source(b);
                    let _label = match label {
                        Label::Label(_) => panic!("Invalid label"),
                        Label::Instruction(value) => *value,
                    };

                    if _a == _b {
                        pc = _label;
                        continue;
                    }
                }
                Instruction::JumpLessThan(a, b, label) => {
                    let _a = self.read_source(a);
                    let _b = self.read_source(b);
                    let _label = match label {
                        Label::Label(_) => panic!("Invalid label"),
                        Label::Instruction(value) => *value,
                    };

                    if _a < _b {
                        pc = _label;
                        continue;
                    }
                }
                Instruction::Move(source, destination) => {
                    let _source = self.read_source(source);
                    let _destination = self.read_destination(destination);

                    self.registers[_destination] = _source;
                }
                Instruction::Call(label) => {
                    let _label = match label {
                        Label::Label(_) => panic!("Invalid label"),
                        Label::Instruction(value) => *value,
                    };

                    self.stack.push(pc + 1);
                    pc = _label;
                    continue;
                }
                Instruction::Return => {
                    if self.stack.is_empty() {
                        // Returning from main works as program exit.
                        return;
                    }

                    pc = self.stack.pop().unwrap();
                    continue;
                }
                Instruction::Time(destination) => {
                    let _destination = self.read_destination(destination);

                    let time = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos();

                    self.registers[_destination] = time as i64;
                }
                Instruction::Syscall(destination, sysno, a1, a2, a3, a4, a5, a6) => {
                    let _destination = self.read_destination(destination);
                    let _sysno = self.read_source(sysno);

                    let _a1 = self.read_optional_source(a1);
                    let _a2 = self.read_optional_source(a2);
                    let _a3 = self.read_optional_source(a3);
                    let _a4 = self.read_optional_source(a4);
                    let _a5 = self.read_optional_source(a5);
                    let _a6 = self.read_optional_source(a6);

                    let ret = unsafe {
                        syscall(
                            _sysno as usize,
                            optional_int_to_usize(_a1),
                            optional_int_to_usize(_a2),
                            optional_int_to_usize(_a3),
                            optional_int_to_usize(_a4),
                            optional_int_to_usize(_a5),
                            optional_int_to_usize(_a6),
                        )
                    };

                    // https://git.musl-libc.org/cgit/musl/tree/src/internal/syscall_ret.c?h=v1.1.15
                    if ret > -4096isize as usize {
                        let errno = -(ret as i32);
                        panic!("syscall failed: {}", errno)
                    } else {
                        self.registers[_destination as usize] = ret as i64;
                    }
                }
            }

            pc += 1;
        }
    }

    fn read_optional_source(&self, param: &Option<crate::instructions::Source>) -> Option<i64> {
        match param {
            Some(value) => Some(self.read_source(value)),
            None => None,
        }
    }

    fn read_source(&self, param: &crate::instructions::Source) -> i64 {
        match param {
            crate::instructions::Source::Data(value) => *value,
            crate::instructions::Source::Address(value) => self.registers[*value],
            crate::instructions::Source::Reference(value) => {
                let referenced_reg = self.registers[*value];
                self.registers[referenced_reg as usize]
            }
        }
    }

    fn read_destination(&self, param: &crate::instructions::Destination) -> usize {
        match param {
            crate::instructions::Destination::Address(value) => *value,
            crate::instructions::Destination::Reference(value) => self.registers[*value] as usize,
        }
    }
}

fn optional_int_to_usize(value: Option<i64>) -> Option<usize> {
    match value {
        Some(value) => Some(value as usize),
        None => None,
    }
}
